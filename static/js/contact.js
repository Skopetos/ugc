const form    = document.getElementById('contact-form');
const success = document.getElementById('form-success');

form?.addEventListener('submit', async e => {
  e.preventDefault();

  const data = Object.fromEntries(new FormData(form));

  try {
    const res = await fetch('https://formspree.io/f/mbdqkgro', {
      method:  'POST',
      headers: { 'Content-Type': 'application/json', 'Accept': 'application/json' },
      body:    JSON.stringify(data),
    });

    if (res.ok) {
      form.style.display    = 'none';
      success.style.display = 'block';
    } else {
      alert('Something went wrong. Please email us directly at tryitaway.info@gmail.com');
    }
  } catch {
    alert('Something went wrong. Please email us directly at tryitaway.info@gmail.com');
  }
});
