function checkMode() {
    const check = document.getElementById('status').checked;
    var mode = parseInt(check ? '1' : '0');
    return mode;
}

async function sendCurrentGuess(player, guess) {
    const mode = checkMode();
    try {
        const state = await player.getCurrentState();
        if (state && state.track_window && state.track_window.current_track) {
            const track_name = state.track_window.current_track.name;

            // Send the guess to the server
            const serverURL = 'http://127.0.0.1:8000/compare';
            const response = await fetch(serverURL, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ guess, track_name, mode })
            });

            if (!response.ok) {
                console.error('Failed to send guess to server:', response.statusText);
            } else {
                const data = await response.json();
                notifyPlayer(data, player);
            }
        }
    } catch (error) {
        console.error('Failed to get current track:', error);
    }
}

async function notifyPlayer(data, player) {
    const state = await player.getCurrentState();
    if (data.same == true) {
        document.getElementById('nextTrack').click();
        console.log('Correct guess!');
        document.getElementById('win_lose_notification').innerHTML = 'Correct!';
    } else {
        console.log('Incorrect guess!');
        console.log('Correct artist: ' + state.track_window.current_track.artists[0].name);
        document.getElementById('win_lose_notification').innerHTML = 'Wrong!';
    }
}

window.onSpotifyWebPlaybackSDKReady = () => {
    const token = localStorage.getItem('access_token');
    const player = new Spotify.Player({
        name: 'Ilan\s Web App',
        getOAuthToken: cb => { cb(token); },
        volume: 0.5
    });

    // Ready
    player.addListener('ready', ({ device_id }) => {
        console.log('Ready with Device ID', device_id);
        player.resume();
    });

    // Not Ready
    player.addListener('not_ready', ({ device_id }) => {
        console.log('Device ID has gone offline', device_id);
    });

    player.addListener('initialization_error', ({ message }) => {
        console.error(message);
    });

    player.addListener('authentication_error', ({ message }) => {
        console.error(message);
    });

    player.addListener('account_error', ({ message }) => {
        console.error(message);
    });


    document.getElementById('toggle_play_button').onclick = function() {
        player.getCurrentState().then(state => {
            if (!state) {
                console.error('User is not playing music through the Web Playback SDK');
                return;
            } else {
                console.log(state);
                console.log("Current track: " + state.track_window.current_track.name);
                console.log("Current artist: " + state.track_window.current_track.artists[0].name);
            }
        })
        player.togglePlay();
    };

    document.getElementById('skip_forward_button').onclick = function() {
        document.getElementById('album_art').style.background = 'none';
        player.nextTrack();
        player.getCurrentState().then(state => {
            if (!state) {
                console.error('User is not playing music through the Web Playback SDK');
                return;
            } else {
                console.log(state);
                // console.log("Current track: " + state.track_window.next_tracks[0].name);
                // console.log("Current artist: " + state.track_window.next_tracks[0].artists[0].name);
            }
        })
    }

    document.getElementById('skip_back_button').onclick = function() {
        player.previousTrack();
        player.getCurrentState().then(state => {
            if (!state) {
                console.error('User is not playing music through the Web Playback SDK');
                return;
            } else {
                console.log(state);
                console.log("Current track: " + state.track_window.previous_tracks[1].name);
                console.log("Current artist: " + state.track_window.previous_tracks[1].artists[0].name);
            }
        })
    }

    // document.getElementById('sendGuess').onclick = function() {
    //     const guess = document.getElementById('songGuessInput').value;
    //     sendCurrentGuess(player, guess);
    // }

    document.getElementById('album_art').onclick = function() {
        player.getCurrentState().then(state => {
            if (!state) {
                console.error('User is not playing music through the Web Playback SDK');
                return;
            } else {
                document.getElementById('album_art').style.background = `url(${state.track_window.current_track.album.images[2].url}) no-repeat center center / cover`;
                console.log(state);
            }
        })
    }

    // connect to player and give buttons ability to control player
    player.connect();

};