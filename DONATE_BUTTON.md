# 💝 Donate Button for Website

## Gumroad Donation Setup

You're using Gumroad's "Pay What You Want" pricing for donations, which is perfect! Here's how to add the donate button to your website.

## HTML Donate Button Code

### Option 1: Simple Link Button (Recommended)
```html
<a href="https://gumroad.com/l/eshu-donate"
   class="donate-button"
   target="_blank"
   rel="noopener">
  💝 Donate
</a>
```

### Option 2: Styled Button with Icon
```html
<a href="https://gumroad.com/l/eshu-donate"
   class="donate-btn"
   target="_blank"
   rel="noopener">
  <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
    <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
  </svg>
  Support the Project
</a>
```

### Option 3: Full Featured Card
```html
<div class="donate-card">
  <h3>💝 Support Eshu</h3>
  <p>Help keep Eshu free and maintained!</p>
  <a href="https://gumroad.com/l/eshu-donate"
     class="btn-primary"
     target="_blank"
     rel="noopener">
    Donate (Pay What You Want)
  </a>
</div>
```

## CSS Styling Examples

### Basic Button Style
```css
.donate-button {
  display: inline-block;
  padding: 12px 24px;
  background: linear-gradient(135deg, #ff69b4 0%, #ff1493 100%);
  color: white;
  text-decoration: none;
  border-radius: 8px;
  font-weight: 600;
  transition: transform 0.2s, box-shadow 0.2s;
  box-shadow: 0 4px 12px rgba(255, 105, 180, 0.3);
}

.donate-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(255, 105, 180, 0.4);
}
```

### Icon Button Style
```css
.donate-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: #ff69b4;
  color: white;
  text-decoration: none;
  border-radius: 8px;
  font-weight: 600;
  transition: all 0.2s;
}

.donate-btn:hover {
  background: #ff1493;
  transform: scale(1.05);
}
```

### Card Style
```css
.donate-card {
  background: white;
  border: 2px solid #ff69b4;
  border-radius: 12px;
  padding: 24px;
  text-align: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.donate-card h3 {
  margin: 0 0 8px 0;
  font-size: 24px;
}

.donate-card p {
  margin: 0 0 16px 0;
  color: #666;
}

.btn-primary {
  display: inline-block;
  padding: 12px 32px;
  background: #ff69b4;
  color: white;
  text-decoration: none;
  border-radius: 8px;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-primary:hover {
  background: #ff1493;
}
```

## Gumroad Widget (Advanced)

Gumroad also provides an embeddable widget that shows up as an overlay:

```html
<script src="https://gumroad.com/js/gumroad.js"></script>
<a class="gumroad-button" href="https://gumroad.com/l/eshu-donate">
  💝 Donate
</a>
```

This will open a beautiful Gumroad overlay instead of redirecting to a new page.

## Website Placement Recommendations

### Header/Navigation
```html
<nav>
  <a href="/">Home</a>
  <a href="/projects">Projects</a>
  <a href="/docs">Docs</a>
  <a href="https://gumroad.com/l/eshu-donate" class="donate-button">💝 Donate</a>
</nav>
```

### Hero Section
```html
<section class="hero">
  <h1>Eshu Apps - Linux Made Easy</h1>
  <p>Universal tools for Linux power users</p>
  <div class="cta-buttons">
    <a href="/get-started" class="btn-primary">Get Started</a>
    <a href="https://gumroad.com/l/eshu-donate" class="btn-secondary">💝 Donate</a>
  </div>
</section>
```

### Footer
```html
<footer>
  <div class="footer-section">
    <h4>Support the Project</h4>
    <p>Help keep Eshu free and maintained</p>
    <a href="https://gumroad.com/l/eshu-donate" class="donate-link">
      💝 Donate (Pay What You Want)
    </a>
  </div>
</footer>
```

## Setting Up on Gumroad

1. **Create the Product**:
   - Go to gumroad.com and create a new product
   - Name: "Support Eshu Development"
   - URL: `gumroad.com/l/eshu-donate`
   - Enable "Pay What You Want" pricing
   - Set minimum: $0 or $1 (your choice)
   - Suggested price: $5 or $10

2. **Product Description**:
```
Support Eshu Development 💝

Thank you for considering supporting Eshu! Your donation helps:
• Keep all Eshu tools free and open source
• Add new features and package managers
• Maintain servers and infrastructure
• Support the developer

Every contribution matters, no matter the size!

Choose any amount you'd like - every dollar helps! ❤️
```

3. **After Purchase Message**:
```
Thank you so much for your support! 🎉

Your contribution helps keep Eshu free for everyone and supports continued development.

Join our community:
• Website: https://eshu-apps.com
• GitHub: https://github.com/eshu-apps
• Support: support@eshu-apps.com

You're awesome! ❤️
```

## Alternative: Ko-fi or Buy Me a Coffee

If you prefer Ko-fi or Buy Me a Coffee instead of Gumroad:

### Ko-fi Button
```html
<a href='https://ko-fi.com/YOUR_USERNAME' target='_blank'>
  <img height='36' style='border:0px;height:36px;'
       src='https://cdn.ko-fi.com/cdn/kofi2.png?v=3'
       border='0' alt='Buy Me a Coffee at ko-fi.com' />
</a>
```

### Buy Me a Coffee Button
```html
<a href="https://www.buymeacoffee.com/YOUR_USERNAME" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png"
       alt="Buy Me A Coffee"
       style="height: 60px !important;width: 217px !important;" />
</a>
```

## Website Integration Checklist

- [ ] Add donate button to navigation bar
- [ ] Add donate section to homepage
- [ ] Add donate link to footer
- [ ] Set up Gumroad product with "Pay What You Want"
- [ ] Test the donation flow
- [ ] Update social media with donation link
- [ ] Add thank you message after donation
- [ ] Consider adding donor recognition (optional)

## Analytics Tracking (Optional)

Track donate button clicks with Google Analytics:

```html
<a href="https://gumroad.com/l/eshu-donate"
   class="donate-button"
   onclick="gtag('event', 'click', {
     'event_category': 'donation',
     'event_label': 'donate_button',
     'value': 1
   });">
  💝 Donate
</a>
```

---

**All donation links are already updated in the GitHub repos!** 🎉
- eshu-installer: Updated README + CLI donate command
- eshu-trace: Updated README
- eshu-shapeshifter: Updated README

Just copy the HTML/CSS above to your website!
