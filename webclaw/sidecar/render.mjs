#!/usr/bin/env node
// Playwright sidecar for webclaw browser rendering.
// Reads JSON requests from stdin, writes rendered HTML to stdout.
// Uses playwright-extra + stealth plugin for anti-bot evasion.
//
// Protocol (one-shot):
//   stdin:  { "url": "https://...", "timeout": 30000, "wait_until": "networkidle" }
//   stdout: { "html": "...", "url": "...", "status": 200 }
//   stderr: diagnostics/errors (not parsed by Rust side)
//
// On error:
//   stdout: { "error": "description" }

// rebrowser-playwright patches CDP leak points that DataDome detects.
// Drop-in replacement for playwright -- same API, fewer detection vectors.
import { chromium } from "rebrowser-playwright";

async function main() {
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  const input = JSON.parse(Buffer.concat(chunks).toString());

  const { url, timeout = 30000, wait_until = "domcontentloaded" } = input;

  if (!url) {
    console.log(JSON.stringify({ error: "missing url" }));
    process.exit(1);
  }

  let browser;
  try {
    browser = await chromium.launch({
      headless: true,
      args: [
        "--disable-blink-features=AutomationControlled",
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-dev-shm-usage",
        "--disable-web-security",
        "--disable-features=IsolateOrigins,site-per-process",
        "--disable-site-isolation-trials",
        "--disable-extensions",
        "--disable-component-extensions-with-background-pages",
        "--disable-default-apps",
        "--no-first-run",
        "--disable-backgrounding-occluded-windows",
        "--disable-renderer-backgrounding",
        "--disable-background-timer-throttling",
        "--disable-ipc-flooding-protection",
      ],
    });

    const context = await browser.newContext({
      userAgent:
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
      viewport: { width: 1920, height: 1080 },
      locale: "en-US",
      timezoneId: "America/New_York",
    });

    const page = await context.newPage();

    // Block WebRTC to prevent IP leaks through STUN
    await page.addInitScript(() => {
      if (navigator.mediaDevices) {
        navigator.mediaDevices.getUserMedia = () => Promise.reject(new Error("Not allowed"));
      }
      window.RTCPeerConnection = undefined;
      window.RTCSessionDescription = undefined;
      window.RTCIceCandidate = undefined;
    });

    // Inject Canvas fingerprint noise (makes each session unique)
    await page.addInitScript(() => {
      const origToBlob = HTMLCanvasElement.prototype.toBlob;
      const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
      const origGetImageData = CanvasRenderingContext2D.prototype.getImageData;

      // Add subtle noise to canvas pixel data
      function addNoise(data) {
        for (let i = 0; i < data.length; i += 4) {
          data[i] = data[i] ^ (Math.random() > 0.99 ? 1 : 0);
        }
        return data;
      }

      CanvasRenderingContext2D.prototype.getImageData = function (...args) {
        const imageData = origGetImageData.apply(this, args);
        addNoise(imageData.data);
        return imageData;
      };

      HTMLCanvasElement.prototype.toBlob = function (cb, ...args) {
        return origToBlob.call(this, (blob) => {
          cb(blob);
        }, ...args);
      };

      HTMLCanvasElement.prototype.toDataURL = function (...args) {
        const ctx = this.getContext("2d");
        if (ctx) {
          const imageData = ctx.getImageData(0, 0, this.width, this.height);
          addNoise(imageData.data);
          ctx.putImageData(imageData, 0, 0);
        }
        return origToDataURL.apply(this, args);
      };
    });

    // Hide WebGL renderer/vendor strings (common fingerprint vector)
    await page.addInitScript(() => {
      const origGetParameter = WebGLRenderingContext.prototype.getParameter;
      WebGLRenderingContext.prototype.getParameter = function (param) {
        // UNMASKED_VENDOR_WEBGL
        if (param === 0x9245) return "Intel Inc.";
        // UNMASKED_RENDERER_WEBGL
        if (param === 0x9246) return "Intel Iris OpenGL Engine";
        return origGetParameter.call(this, param);
      };
      if (typeof WebGL2RenderingContext !== "undefined") {
        const origGetParameter2 = WebGL2RenderingContext.prototype.getParameter;
        WebGL2RenderingContext.prototype.getParameter = function (param) {
          if (param === 0x9245) return "Intel Inc.";
          if (param === 0x9246) return "Intel Iris OpenGL Engine";
          return origGetParameter2.call(this, param);
        };
      }
    });

    const response = await page.goto(url, {
      waitUntil: wait_until,
      timeout,
    });

    // Wait for JS frameworks to render content.
    // domcontentloaded fires fast, then we give SPAs time to hydrate.
    // networkidle hangs on sites with infinite tracking pixels (Amazon).
    await page.waitForTimeout(3000);

    const html = await page.content();
    const finalUrl = page.url();
    const status = response ? response.status() : 0;

    console.log(JSON.stringify({ html, url: finalUrl, status }));
  } catch (err) {
    console.log(JSON.stringify({ error: err.message }));
    process.exit(1);
  } finally {
    if (browser) await browser.close();
  }
}

main();
