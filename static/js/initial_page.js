const generateRandomString = (length) => {
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  const values = crypto.getRandomValues(new Uint8Array(length));
  return values.reduce((acc, x) => acc + possible[x % possible.length], "");
}

const sha256 = async (plain) => {
  const encoder = new TextEncoder()
  const data = encoder.encode(plain)
  return window.crypto.subtle.digest('SHA-256', data)
}

const base64encode = (input) => {
  return btoa(String.fromCharCode(...new Uint8Array(input)))
    .replace(/=/g, '')
    .replace(/\+/g, '-')
    .replace(/\//g, '_');
}

const authorizeSpotify = async () => {

  const codeVerifier = generateRandomString(128);
  const hashed = await sha256(codeVerifier);
  const codeChallenge = base64encode(hashed);

  const clientId = "72ae29791cf14328b43c1c1c03fa19e8";
  const redirctUri = "http://127.0.0.1:8000";

  const scope = "user-read-private user-read-email";
  const authUrl = new URL("https://accounts.spotify.com/authorize");

  localStorage.setItem("code_verifier", codeVerifier);

  const params = {
    response_type: 'code',
    client_id: clientId,
    scope,
    code_challenge_method: 'S256',
    codeChallenge: codeChallenge,
    redirect_uri: redirctUri,
  }

  authUrl.search = new URLSearchParams(params).toString();
  window.location.href = authUrl.toString();
}

document.addEventListener('DOMContentLoaded', () => {
  // add event listener to the login button
  // when the button is clicked, the user will be redirected to the Spotify login page
  // when the button is clicked, isPlayGameButtonHidden will be set to false
  // (so that the game button is visible when user is redirect back to the app)
  const button = document.getElementById('login');
  button.addEventListener('click', authorizeSpotify);
  button.addEventListener('click', function() {
    localStorage.setItem('isPlayGameButtonHidden', 'false');
  });

  // check if the user is already logged in
  // if the user is already logged in, the login button will be hidden
  // and the game button will be visible
  var isPlayGameButtonHidden = localStorage.getItem('isPlayGameButtonHidden');
  var codeVerifier = localStorage.getItem('code_verifier');

  if (isPlayGameButtonHidden === 'false' && codeVerifier !== null) {
    document.getElementById('login').style.visibility = 'hidden';
    document.getElementById('game').style.visibility = 'visible';
  } else {
    document.getElementById('login').style.visibility = 'visible';
    document.getElementById('game').style.visibility = 'hidden';
  }

  
});