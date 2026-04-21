# BDO Ping Monitor — Design PRD

## 1. Overview
BDO Ping Monitor is a desktop latency monitoring tool for Black Desert Online players. The UI should feel like a premium gaming utility: dark, technical, and highly legible under pressure.

## 2. Color Palette
- Background Primary: #0a0a0f (deep void black)
- Background Secondary: #12121a (elevated surface)
- Background Tertiary: #1a1a24 (card/hover surface)
- Accent Primary: #e94560 (crimson red — BDO energy)
- Accent Secondary: #ff9f43 (amber — warning/medium latency)
- Accent Tertiary: #2ed573 (neon green — good latency)
- Text Primary: #f0f0f5 (off-white)
- Text Secondary: #8a8a9a (muted gray)
- Border: rgba(255, 255, 255, 0.06)

## 3. Typography
- Primary Font: Inter, system-ui, sans-serif
- Monospace Font: JetBrains Mono, Fira Code, monospace (for ping numbers)
- Scale:
  - Title: 20px, font-weight 700
  - Server Name: 14px, font-weight 600
  - Ping Value: 24px, font-weight 700, monospace
  - Body: 13px, font-weight 400
  - Caption: 11px, font-weight 500, uppercase, letter-spacing 0.05em

## 4. Layout
- Window: 450×620px (fixed min, resizable)
- Padding: 24px outer, 16px inner
- Structure:
  1. Header (logo + title + status)
  2. Server List (scrollable)
  3. Mini Chart (last 60s history)
  4. Footer (last update + controls)

## 5. Components

### Server Card
- Background: Background Secondary
- Border: 1px solid Border
- Border-radius: 12px
- Padding: 16px
- Shadow: 0 4px 24px rgba(0,0,0,0.4)
- Layout: Flex row, space-between
- Left: Server name + region tag
- Right: Ping value + status dot
- Hover: Border color transitions to Accent Primary at 30% opacity

### Status Dot
- Size: 8px
- Colors: Green #2ed573, Amber #ff9f43, Red #ff4d4d
- Pulse animation on active measurement

### Ping Value
- Monospace font
- Color thresholds:
  - < 50ms: Green
  - 50-100ms: Amber
  - > 100ms: Red
  - Timeout: Gray with strikethrough

### Sparkline Chart
- Height: 40px
- Stroke: Accent Primary
- Fill: gradient from Accent Primary at 20% to transparent
- No axes, pure visual trend

## 6. Animations & Interactions
- Card Hover: transform: translateY(-2px), border glow, 200ms ease-out
- Ping Update: Number rolls with 150ms fade
- Status Dot: Pulse scale(1) → scale(1.4) → scale(1), 1.5s infinite
- Loading Skeleton: Shimmer gradient sweep, 1.5s infinite
- Window Open: Fade in + scale from 0.98, 300ms ease-out

## 7. Scrollbar
- Width: 6px
- Track: transparent
- Thumb: rgba(255,255,255,0.15), border-radius 3px
- Thumb Hover: rgba(255,255,255,0.25)

## 8. Glassmorphism Touches
- Header: backdrop-filter: blur(20px), semi-transparent background
- Modal/Overlay: backdrop-filter: blur(12px)