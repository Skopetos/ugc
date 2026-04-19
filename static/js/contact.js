const form    = document.getElementById('contact-form');
const success = document.getElementById('form-success');

form?.addEventListener('submit', async e => {
  e.preventDefault();

  const data = Object.fromEntries(new FormData(form));

  try {
    const res = await fetch('/api/contact', {
      method:  'POST',
      headers: { 'Content-Type': 'application/json' },
      body:    JSON.stringify(data),
    });

    if (res.ok) {
      form.style.display    = 'none';
      success.style.display = 'block';
    } else {
      // Fallback to mailto if server returns error
      window.location.href = `mailto:tryitaway.info@gmail.com?subject=Inquiry from ${encodeURIComponent(data.name)}&body=${encodeURIComponent(data.message)}`;
    }
  } catch {
    // Offline / server not running — open mail client
    window.location.href = `mailto:tryitaway.info@gmail.com?subject=Inquiry from ${encodeURIComponent(data.name)}&body=${encodeURIComponent(data.message)}`;
  }
});
