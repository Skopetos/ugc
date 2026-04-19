// Register Gmail as mailto handler on desktop
if (navigator.registerProtocolHandler) {
  try {
    navigator.registerProtocolHandler(
      'mailto',
      'https://mail.google.com/mail/?extsrc=mailto&url=%s',
      'Gmail'
    );
  } catch(e) {}
}

// Mobile nav toggle
const hamburger = document.querySelector('.hamburger');
const navLinks  = document.querySelector('.nav-links');

if (hamburger) {
  hamburger.addEventListener('click', () => {
    hamburger.classList.toggle('open');
    navLinks.classList.toggle('open');
  });
}

// Close mobile nav on link click
document.querySelectorAll('.nav-links a').forEach(link => {
  link.addEventListener('click', () => {
    hamburger?.classList.remove('open');
    navLinks?.classList.remove('open');
  });
});
