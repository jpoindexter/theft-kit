# Animation & Motion Guidelines

## Physics-Based Motion

Real-world physics makes animations feel natural and intuitive.

### Easing Functions

| Easing | Use Case | CSS Equivalent |
|--------|----------|----------------|
| Ease Out | UI elements entering screen | `cubic-bezier(0, 0, 0.2, 1)` |
| Ease In | UI elements exiting screen | `cubic-bezier(0.4, 0, 1, 1)` |
| Ease In-Out | Moving within screen | `cubic-bezier(0.4, 0, 0.2, 1)` |
| Spring | Playful, bouncy elements | `cubic-bezier(0.34, 1.56, 0.64, 1)` |

### Movement Magnitudes

```
Small interactions (buttons, toggles): 2-8px
Medium interactions (cards, panels): 8-24px  
Large interactions (modals, pages): 24-100px
Scale effects: 0.95x - 1.05x
Rotation: 5° - 15° for subtle, up to 45° for emphasis
```

### Duration Guidelines

```
Micro-interactions (hover states): 100-150ms
Standard transitions: 200-300ms
Complex sequences: 400-600ms
Page transitions: 300-500ms
Ambient motion: 8s-20s loops
```

### Entrance Animations

Elements entering the screen should feel alive:

```css
@keyframes slideUpFade {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
```

**Stagger Patterns:**
```javascript
// Stagger children for sequential reveals
const staggerDelay = 50; // ms between each item
items.forEach((item, index) => {
  item.style.animationDelay = `${index * staggerDelay}ms`;
});
```

### Hover & Interaction States

Hover states should feel responsive and purposeful:

```css
.button {
  transition: transform 150ms ease-out, 
              box-shadow 150ms ease-out;
}

.button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.button:active {
  transform: translateY(0) scale(0.98);
  transition-duration: 50ms;
}
```

### Scroll-Triggered Animations

Animate elements as they enter the viewport:

```javascript
const observer = new IntersectionObserver((entries) => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      entry.target.classList.add('animate-in');
    }
  });
}, { threshold: 0.1 });
```

### Performance Rules

**DO:**
- Animate `transform` and `opacity` only (GPU accelerated)
- Use `will-change` sparingly and remove after animation
- Throttle scroll events to 60fps
- Use CSS animations for simple transitions
- Test on actual mobile devices

**DON'T:**
- Animate `width`, `height`, `top`, `left` (causes reflow)
- Use blur filters during scroll
- Run animations on unmounted components
- Ignore `prefers-reduced-motion`

### Accessibility

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

## Scroll Behavior

### Natural Scrolling

- Use momentum scrolling on touch devices: `-webkit-overflow-scrolling: touch`
- Avoid hijacking scroll behavior unless absolutely necessary
- Parallax effects should be subtle (max 20% speed difference)
- Sticky headers should appear smoothly, not abruptly

### Parallax Guidelines

```javascript
// Smooth parallax with RAF
let ticking = false;
window.addEventListener('scroll', () => {
  if (!ticking) {
    requestAnimationFrame(() => {
      updateParallax();
      ticking = false;
    });
    ticking = true;
  }
});
```

## Micro-interactions

### Button Feedback

Every button press should provide immediate feedback:

```css
.btn {
  position: relative;
  transition: all 150ms ease-out;
}

.btn::after {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: inherit;
  box-shadow: 0 0 0 0 currentColor;
  opacity: 0.3;
  transition: box-shadow 200ms ease-out;
}

.btn:active::after {
  box-shadow: 0 0 0 4px currentColor;
}
```

### Loading States

Replace buttons with loading indicators during async operations:

```jsx
<button disabled={isLoading}>
  {isLoading ? <Spinner size="small" /> : 'Submit'}
</button>
```

### Form Validation

Animate validation states smoothly:

```css
.input-error {
  animation: shake 300ms ease-in-out;
  border-color: var(--color-error);
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-4px); }
  75% { transform: translateX(4px); }
}
```

## Animation Libraries

### Recommended Tools

| Library | Best For | Size |
|---------|----------|------|
| Framer Motion | React component animations | ~38kb |
| GSAP | Complex timelines, scroll triggers | ~25kb core |
| Lottie | Complex vector animations | ~60kb |
| CSS Animations | Simple transitions | 0kb |

### Framer Motion Patterns

```jsx
import { motion, AnimatePresence } from 'framer-motion';

// Page transitions
<motion.div
  initial={{ opacity: 0, y: 20 }}
  animate={{ opacity: 1, y: 0 }}
  exit={{ opacity: 0, y: -20 }}
  transition={{ duration: 0.3, ease: 'easeOut' }}
>
  {children}
</motion.div>

// Stagger children
const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: {
      staggerChildren: 0.1
    }
  }
};

const item = {
  hidden: { opacity: 0, y: 20 },
  show: { opacity: 1, y: 0 }
};
```

### CSS vs JS Animations

Use CSS when:
- Simple state changes (hover, focus, active)
- Properties that can be transitioned (opacity, transform)
- No complex sequencing needed
- Best performance is critical

Use JS when:
- Complex choreography required
- Need to chain animations
- Dynamic values based on user input
- Spring physics needed
