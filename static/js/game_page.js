async function getProfile() {
    let accessToken = localStorage.getItem('access_token');

    const response = await fetch('https://api.spotify.com/v1/me', {
        headers: {
            'Authorization': `Bearer ${accessToken}`
        }
    });
    
    const data = await response.json();
    console.log(data);
}

function debounce(func, wait) {
    let timeout;
    return function(...args) {
        const context = this;
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(context, args), wait);
    };
}

async function fetchSuggestions(event) {
    const query = event.target.value.trim();
    if (!query) {
        return;
    }

    try {
        const response = await fetch(`https://api.spotify.com/v1/search?q=${encodeURIComponent(query)}&type=track&limit=5`, {
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('access_token')}`
            }
        });

        if (response.ok) {
            const data = await response.json();
            // console.log(data.tracks.items.map(item => item.name + ' - ' + item.artists[0].name));
            displaySuggestions(data.tracks.items)
        } else {
            console.error('Failed to fetch suggestions:', response.statusText);
        }
    } catch (error) {
        console.error('Failed to fetch suggestions:', error);
    }
}

function displaySuggestions(tracks) {
    const suggestions = document.getElementById('suggestions');
    suggestions.innerHTML = '';
    tracks.forEach(track => {
        const li = document.createElement('li');
        li.textContent = `${track.name} - ${track.artists.map(artist => artist.name).join(', ')}`;
        suggestions.appendChild(li);
    })
}


document.addEventListener('DOMContentLoaded', () => {
    getProfile();
    // call fetchSuggestions when the user types in the input field
    document.getElementById('songGuessInput').addEventListener('keydown', debounce(fetchSuggestions, 300));

    // try guess and clear input field when user presses enter
    document.getElementById('songGuessInput').addEventListener('keypress', function(e){
        if(e.key === 'Enter'){
            document.getElementById('sendGuess').click();
            document.getElementById('songGuessInput').value = '';
            displaySuggestions([]);
        }
    });
});