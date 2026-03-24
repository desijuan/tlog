async function loginBtnHandler() {
  const nameInput = document.getElementById('username');
  const name = nameInput.value.trim();
  if (!name) {
    alert('Por favor ingrese su usuario.');
    nameInput.focus();
    return;
  }

  const pwdInput = document.getElementById('password');
  const pwd = pwdInput.value.trim();
  if (!pwd) {
    alert('Por favor ingrese su contraseña.');
    pwdInput.focus();
    return;
  }

  let response;
  try {
    response = await fetch('/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, pwd })
    });
  } catch (e) {
    console.error('fetch failed:', e);
    return;
  }

  if (!response.ok) {
    if (response.status === 401) {
      alert('Nombre de usuario y/o contraseña incorrectos.');
    } else {
      alert('Error inesperado.');
    }
    return;
  }

  window.location.href = '/app/panel-usuario';
}

document.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') {
    loginBtnHandler();
  }
});
