# REQ-007 Implementation Plan: 3-Layer Parallax Starfield

## Based on Lessons Learned

### Previous Failures
1. ❌ Inline styles in Yew's html! macro broke rendering
2. ❌ Stars too small (1-3px) - invisible on dark backgrounds
3. ❌ Background covered only viewport, not full page
4. ❌ Parallax transforms applied incorrectly
5. ❌ Sections had opaque backgrounds blocking stars

---

## Revised Requirements

### Visual Design
- **3 star layers** with parallax depth effect
- **Semi-opaque background** (rgba(10,10,15, 0.95)) on content sections
- **Stars visible through** semi-transparent areas
- **Warm bottom glow** (campfire effect)

### Star Specifications
| Layer | Count | Size | Opacity | Scroll Speed | Z-Index |
|-------|-------|------|---------|--------------|---------|
| Back  | 30    | 4-6px| 40%     | 0.05x        | -3      |
| Mid   | 50    | 5-7px| 60%     | 0.15x        | -2      |
| Front | 40    | 6-8px| 80%     | 0.25x        | -1      |

---

## Implementation Approach: CSS-First Strategy

### Step 1: Static CSS Background (Single Layer)
**Goal:** Verify stars render at all

```css
/* styles.css */
.starfield-bg {
  position: fixed;
  inset: 0;
  z-index: -1;
  background: linear-gradient(to bottom, #0a0a0f 0%, #151520 50%, #1a1510 100%);
}

/* 10 test stars */
.star-1 { position: absolute; top: 10%; left: 20%; width: 6px; height: 6px; background: white; border-radius: 50%; box-shadow: 0 0 8px 4px rgba(255,255,255,0.5); }
/* ... etc */
```

**Success Criteria:** Stars visible in screenshot

---

### Step 2: Add Semi-Opaque Sections
**Goal:** Verify stars show through content

```rust
// In each section component:
style="background-color: rgba(10, 10, 15, 0.95);"
```

**Success Criteria:** Stars visible through semi-transparent backgrounds

---

### Step 3: Add Second Star Layer
**Goal:** Test multiple layers

- Copy Step 1 approach
- Add second CSS class with different positions
- Use z-index: -2 for back layer

**Success Criteria:** Both layers visible

---

### Step 4: Add Parallax Scroll Effect
**Goal:** Make layers move at different speeds

**Approach:** Minimal JavaScript
```javascript
// In use_effect
window.addEventListener('scroll', () => {
  const scrollY = window.scrollY;
  document.querySelector('.stars-back').style.transform = `translateY(${scrollY * 0.05}px)`;
  document.querySelector('.stars-mid').style.transform = `translateY(${scrollY * 0.15}px)`;
  document.querySelector('.stars-front').style.transform = `translateY(${scrollY * 0.25}px)`;
});
```

---

### Step 5: Ember Particles
**Goal:** Add warmth animation

```css
.ember {
  position: absolute;
  width: 8px;
  height: 8px;
  background: radial-gradient(circle, #fa8805 0%, transparent 70%);
  animation: rise 6s ease-out infinite;
}

@keyframes rise {
  from { transform: translateY(0); opacity: 0; }
  10% { opacity: 1; }
  to { transform: translateY(-40vh); opacity: 0; }
}
```

---

## Verification Checklist

- [ ] Step 1: Stars visible (simpler = better)
- [ ] Step 2: Stars visible through sections (semi-opaque bg)
- [ ] Step 3: Multiple layers visible (z-index working)
- [ ] Step 4: Parallax effect visible (scroll test)
- [ ] Step 5: Embers animating (warmth visible)

---

## Risk Mitigation

**If Step 1 fails (stars not visible):**
- Increase size to 10px temporarily
- Change color to red (#ff0000) to confirm rendering
- Check computed styles in browser devtools

**If Step 2 fails (not visible through sections):**
- Reduce opacity to 0.8 or lower
- Check z-index stacking
- Verify sections don't create new stacking contexts

**If Step 4 fails (no parallax):**
- Use CSS `background-attachment: fixed` as fallback
- Accept static stars if parallax is too complex

---

## Acceptance Criteria (Updated)

1. ✅ 3 layers of stars visible on dark background
2. ✅ Stars visible through semi-opaque content sections
3. ✅ Layers move at different speeds on scroll (parallax)
4. ✅ Warm ember particles rise from bottom
5. ✅ Screenshot confirms visibility (not just DOM presence)

---

## Decision Points

| Stage | Decision | Criteria |
|-------|----------|----------|
| After Step 1 | Continue to Step 2? | Stars visible in screenshot |
| After Step 2 | Continue to Step 3? | Stars visible through sections |
| After Step 3 | Continue to Step 4? | Multiple layers visible |
| After Step 4 | Continue to Step 5? | Parallax effect visible |

**Stop and reassess** if any step fails - don't proceed until previous step works.
