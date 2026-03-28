let userName = null;
let fichajes = null;
let nextFichajeKind = null;

async function ficharBtnHandler() {
  if (nextFichajeKind === null) {
    return;
  }

  let response;
  try {
    response = await fetch('/api/fichajes', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ kind: nextFichajeKind })
    });
  } catch (e) {
    console.error('fetch failed:', e);
    return;
  }

  if (!response.ok) {
    alert('Algo salió mal...');
    return;
  }

  updateFichajes();
}

function updateClock() {
  const now = new Date();

  document.querySelector('.current-time').textContent =
    now.toLocaleTimeString('es-AR', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false
    });

  document.querySelector('.current-date').textContent =
    now.toLocaleDateString('es-AR', {
      weekday: 'long',
      day: 'numeric',
      month: 'long',
      year: 'numeric'
    });
}

function startClock() {
  updateClock();

  const now = new Date();
  const msToNextMinute =
    (60 - now.getSeconds()) * 1000 - now.getMilliseconds();

  setTimeout(() => {
    updateClock();
    setInterval(updateClock, 60000);
  }, msToNextMinute);
}

async function updateName() {
  let response;
  try {
    response = await fetch('/api/users');
  } catch (e) {
    console.error('fetch failed:', e);
    return;
  }

  if (!response.ok) {
    console.error('HTTP error:', response.status);
    return;
  }

  const data = await response.json();
  const name = data.user.name;
  userName = name.charAt(0).toUpperCase() + name.slice(1);

  document.getElementById('user-name').textContent = userName;
}

async function updateFichajes() {
  let response;
  try {
    response = await fetch('/api/fichajes');
  } catch (e) {
    console.error('fetch failed:', e);
    return;
  }

  if (!response.ok) {
    console.error('HTTP error:', response.status);
    return;
  }

  const data = await response.json();
  fichajes = data.fichajes;

  const mainContent = document.querySelector('.main-content');

  document.querySelectorAll('.clock-entry').forEach(e => e.remove());

  const grouped = pairFichajes(fichajes);

  for (const day of grouped) {
    const entryDiv = document.createElement('div');
    entryDiv.className = 'clock-entry';

    const headerDiv = document.createElement('div');
    headerDiv.className = 'clock-entry-header';

    const entradaSpan = document.createElement('span');
    entradaSpan.className = 'clock-entry-time';
    entradaSpan.textContent = `Entrada: ${day.in || '--:--'}`;

    const salidaSpan = document.createElement('span');
    salidaSpan.className = 'clock-entry-time';
    salidaSpan.textContent = `Salida: ${day.out || '--:--'}`;

    headerDiv.appendChild(entradaSpan);
    headerDiv.appendChild(salidaSpan);

    const dateDiv = document.createElement('div');
    dateDiv.className = 'clock-entry-date';

    const dateObj = new Date(day.ts * 1000);

    dateDiv.textContent = dateObj.toLocaleDateString('es-AR', {
      weekday: 'long',
      day: 'numeric',
      month: 'long',
      year: 'numeric'
    });

    entryDiv.appendChild(headerDiv);
    entryDiv.appendChild(dateDiv);

    mainContent.appendChild(entryDiv);
  }

  const maxFichajeKind =
    fichajes.length === 0
      ? "OUT"
      : fichajes.reduce((max, current) =>
        current.ts > max.ts ? current : max
      ).kind;

  let ficharBtnLabel
  switch (maxFichajeKind) {
    case 'OUT':
      nextFichajeKind = 'IN'
      ficharBtnLabel = 'Fichar Entrada';
      break;

    case 'IN':
      nextFichajeKind = 'OUT'
      ficharBtnLabel = 'Fichar Salida';
      break;

    default:
      console.warn('Unknown kind:', maxFichajeKind);
      return;
  }

  document.getElementById('fichar-btn').textContent = ficharBtnLabel;
}

function pairFichajes(fichajes) {
  // Sort numerically ascending
  fichajes.sort((a, b) => a.ts - b.ts);

  const result = [];
  let currentIn = null;

  for (const f of fichajes) {
    if (f.kind === 'IN') {
      currentIn = f;
    } else if (f.kind === 'OUT' && currentIn) {
      result.push(buildPair(currentIn, f));
      currentIn = null;
    }
  }

  if (currentIn) {
    result.push(buildPair(currentIn, null));
  }

  // Newest first
  result.sort((a, b) => b.ts - a.ts);

  return result;
}

function buildPair(inF, outF) {
  const dateIn = new Date(inF.ts * 1000);
  const dateOut = outF ? new Date(outF.ts * 1000) : null;

  const timeIn = dateIn.toLocaleTimeString('es-AR', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });

  const timeOut = dateOut
    ? dateOut.toLocaleTimeString('es-AR', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false
    })
    : null;

  return {
    ts: inF.ts,   // keep raw timestamp for sorting & date rendering
    in: timeIn,
    out: timeOut
  };
}

document.addEventListener('DOMContentLoaded', startClock);
document.addEventListener('DOMContentLoaded', updateName);
document.addEventListener('DOMContentLoaded', updateFichajes);
