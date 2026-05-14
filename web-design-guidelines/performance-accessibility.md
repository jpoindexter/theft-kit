# Performance & Accessibility

## Performance Guidelines

### Core Web Vitals Targets

| Metric | Target | Description |
|--------|--------|-------------|
| LCP | < 2.5s | Largest Contentful Paint |
| FID | < 100ms | First Input Delay |
| CLS | < 0.1 | Cumulative Layout Shift |
| FCP | < 1.8s | First Contentful Paint |
| TTFB | < 600ms | Time to First Byte |

### Image Optimization

**Format Selection:**
- Photos: WebP or AVIF (with JPEG fallback)
- Icons/Simple graphics: SVG
- Transparency needed: WebP or PNG
- Animations: WebP (instead of GIF)

**Responsive Images:**
```html
<img 
  srcset="image-400.jpg 400w, image-800.jpg 800w, image-1200.jpg 1200w"
  sizes="(max-width: 600px) 400px, (max-width: 1000px) 800px, 1200px"
  src="image-800.jpg"
  alt="Description"
/>
```

**Lazy Loading:**
```html
<img src="image.jpg" loading="lazy" alt="Description" />
```

### Code Splitting

```jsx
// React.lazy for route-based splitting
const Dashboard = React.lazy(() => import('./Dashboard'));
const Settings = React.lazy(() => import('./Settings'));

// Dynamic imports for heavy components
const HeavyChart = React.lazy(() => import('./HeavyChart'));

function App() {
  return (
    <Suspense fallback={<Loading />}>
      <Routes>
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </Suspense>
  );
}
```

### CSS Optimization

**Critical CSS:**
- Inline critical CSS in `<head>`
- Load non-critical CSS asynchronously
- Use `media` queries to prevent render-blocking

**Tailwind Optimization:**
```javascript
// tailwind.config.js
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  purge: {
    enabled: process.env.NODE_ENV === 'production',
    content: ['./src/**/*.{js,jsx,ts,tsx}']
  }
};
```

### Font Loading

```html
<!-- Preconnect to font domain -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>

<!-- Use font-display: swap -->
<link href="https://fonts.googleapis.com/css2?family=...&display=swap" rel="stylesheet">
```

```css
@font-face {
  font-family: 'Custom Font';
  src: url('font.woff2') format('woff2');
  font-weight: 400;
  font-display: swap; /* Show fallback font immediately */
}
```

### Caching Strategies

**Static Assets:**
```
Cache-Control: public, max-age=31536000, immutable
```

**HTML Documents:**
```
Cache-Control: no-cache (or short cache with revalidation)
```

### Bundle Size Monitoring

```javascript
// webpack-bundle-analyzer
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;

module.exports = {
  plugins: [
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      openAnalyzer: false
    })
  ]
};
```

## Accessibility (a11y)

### Semantic HTML

```html
<!-- Good: Semantic structure -->
<header>
  <nav aria-label="Main navigation">
    <ul>
      <li><a href="/">Home</a></li>
    </ul>
  </nav>
</header>

<main>
  <article>
    <h1>Article Title</h1>
    <p>Content...</p>
  </article>
</main>

<aside>
  <h2>Related Links</h2>
</aside>

<footer>
  <p>&copy; 2024 Company</p>
</footer>
```

### ARIA Attributes

```html
<!-- Landmark regions -->
<div role="banner">Header content</div>
<div role="navigation" aria-label="Main">Nav content</div>
<div role="main">Main content</div>
<div role="complementary">Sidebar content</div>
<div role="contentinfo">Footer content</div>

<!-- Live regions for dynamic content -->
<div aria-live="polite" aria-atomic="true">
  {notificationMessage}
</div>

<!-- Navigation roles -->
<nav aria-label="Main navigation">
<nav aria-label="Breadcrumb">
```

### Keyboard Navigation

**Focus Management:**
```css
/* Visible focus indicators */
:focus-visible {
  outline: 2px solid #0066cc;
  outline-offset: 2px;
}

/* Skip links */
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: #000;
  color: #fff;
  padding: 8px;
  z-index: 100;
}

.skip-link:focus {
  top: 0;
}
```

**Focus Trap in Modals:**
```javascript
// Trap focus within modal
useEffect(() => {
  if (isOpen) {
    const focusableElements = modalRef.current.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];
    
    firstElement.focus();
    
    const handleTabKey = (e) => {
      if (e.key === 'Tab') {
        if (e.shiftKey && document.activeElement === firstElement) {
          e.preventDefault();
          lastElement.focus();
        } else if (!e.shiftKey && document.activeElement === lastElement) {
          e.preventDefault();
          firstElement.focus();
        }
      }
    };
    
    modalRef.current.addEventListener('keydown', handleTabKey);
    return () => modalRef.current?.removeEventListener('keydown', handleTabKey);
  }
}, [isOpen]);
```

### Color & Contrast

**Requirements:**
- Normal text: 4.5:1 minimum contrast
- Large text (18px+): 3:1 minimum contrast
- UI components: 3:1 minimum contrast

**Testing Tools:**
- Chrome DevTools Lighthouse
- axe DevTools extension
- WAVE (Web Accessibility Evaluation Tool)
- Color Contrast Analyzer

**Don't Rely on Color Alone:**
```html
<!-- Bad: Only color indicates state -->
<span class="text-red-500">Error</span>

<!-- Good: Color + icon + text -->
<span class="text-red-600 flex items-center">
  <ErrorIcon className="mr-1" />
  Error: Please fix the field
</span>
```

### Screen Reader Considerations

**Alt Text:**
```html
<!-- Descriptive for content images -->
<img src="chart.jpg" alt="Bar chart showing 50% increase in sales from Q1 to Q2" />

<!-- Empty for decorative images -->
<img src="decoration.jpg" alt="" />

<!-- Describe function for icons -->
<button aria-label="Search">
  <SearchIcon />
</button>
```

**Hidden Content:**
```css
/* Visually hidden but accessible to screen readers */
.visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* Hidden from everyone */
.hidden {
  display: none;
}

/* Hidden from screen readers but visible */
aria-hidden="true"
```

### Form Accessibility

**Labels:**
```html
<!-- Explicit label association -->
<label for="username">Username</label>
<input id="username" type="text" />

<!-- Implicit label (wrapped) -->
<label>
  Username
  <input type="text" />
</label>

<!-- Aria-label when label is not visible -->
<input type="search" aria-label="Search products" />
```

**Error Handling:**
```html
<input 
  aria-invalid={hasError}
  aria-describedby={hasError ? 'email-error' : undefined}
/>
{hasError && (
  <p id="email-error" role="alert" className="text-red-600">
    Please enter a valid email address
  </p>
)}
```

**Required Fields:**
```html
<input required aria-required="true" />
<!-- Or use aria-describedby for required indicator -->
<span id="required-desc" className="visually-hidden">required</span>
<input aria-describedby="required-desc" />
```

### Motion & Animation

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

### Testing Checklist

**Automated Testing:**
- [ ] Run Lighthouse accessibility audit
- [ ] Test with axe DevTools
- [ ] Validate HTML with W3C validator
- [ ] Check color contrast ratios

**Manual Testing:**
- [ ] Navigate with keyboard only
- [ ] Test with screen reader (NVDA, JAWS, VoiceOver)
- [ ] Zoom to 200%, check readability
- [ ] Test with reduced motion enabled
- [ ] Verify focus indicators are visible

**Common Issues:**
- Missing alt text
- Insufficient color contrast
- Missing form labels
- Non-keyboard accessible elements
- Missing focus indicators
- Improper heading hierarchy
- Auto-playing media without controls

## Testing Tools

| Tool | Purpose |
|------|---------|
| Lighthouse | Performance & accessibility audit |
| axe DevTools | Accessibility testing |
| WebPageTest | Performance analysis |
| GTmetrix | Performance monitoring |
| WAVE | Web accessibility evaluation |
| NVDA/VoiceOver | Screen reader testing |
