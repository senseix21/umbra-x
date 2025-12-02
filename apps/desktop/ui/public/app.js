// Wait for Tauri API to be available
if (!window.__TAURI__) {
  document.body.innerHTML = '<div style="color: white; padding: 40px; text-align: center;"><h1>⚠️ Tauri API not available</h1><p>Make sure you are running with: cargo run</p></div>';
  throw new Error('Tauri API not available');
}

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// State
let currentPeer = null;
let peers = new Map();
let myPeerId = null;

// DOM Elements
const connectBtn = document.getElementById('connect-btn');
const connectModal = document.getElementById('connect-modal');
const connectSubmit = document.getElementById('connect-submit');
const cancelBtn = document.getElementById('cancel-btn');
const peerAddrInput = document.getElementById('peer-addr');
const chatList = document.getElementById('chat-list');
const messages = document.getElementById('messages');
const messageInput = document.getElementById('message-input');
const sendBtn = document.getElementById('send-btn');
const peerInfo = document.getElementById('peer-info');
const peerName = document.getElementById('peer-name');
const myPeerIdEl = document.getElementById('my-peer-id');
const copyIdBtn = document.getElementById('copy-id-btn');
const listenAddrsEl = document.getElementById('listen-addrs');
const fullMultiaddrEl = document.getElementById('full-multiaddr');
const copyAddrBtn = document.getElementById('copy-addr-btn');

// Event Listeners
connectBtn.addEventListener('click', () => {
  connectModal.classList.remove('hidden');
  peerAddrInput.focus();
});

cancelBtn.addEventListener('click', () => {
  connectModal.classList.add('hidden');
  peerAddrInput.value = '';
});

copyIdBtn.addEventListener('click', async () => {
  if (!myPeerId) return;
  
  try {
    await navigator.clipboard.writeText(myPeerId);
    copyIdBtn.textContent = '✅ Copied!';
    setTimeout(() => {
      copyIdBtn.textContent = '📋 Copy ID';
    }, 2000);
  } catch (err) {
    console.error('Failed to copy:', err);
  }
});

copyAddrBtn.addEventListener('click', async () => {
  const addr = fullMultiaddrEl.textContent;
  if (!addr) return;
  
  try {
    await navigator.clipboard.writeText(addr);
    copyAddrBtn.textContent = '✅ Copied!';
    setTimeout(() => {
      copyAddrBtn.textContent = '📋 Copy Address';
    }, 2000);
  } catch (err) {
    console.error('Failed to copy:', err);
  }
});

// Click peer ID to copy
myPeerIdEl.addEventListener('click', () => {
  if (myPeerId) {
    copyIdBtn.click();
  }
});

connectSubmit.addEventListener('click', async () => {
  const addr = peerAddrInput.value.trim();
  if (!addr) return;
  
  try {
    await invoke('connect_peer', { peerInput: addr });
    connectModal.classList.add('hidden');
    peerAddrInput.value = '';
  } catch (err) {
    alert('Failed to connect: ' + err);
  }
});

sendBtn.addEventListener('click', sendMessage);
messageInput.addEventListener('keypress', (e) => {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
});

async function sendMessage() {
  const text = messageInput.value.trim();
  if (!text || !currentPeer) return;
  
  try {
    await invoke('send_message', { 
      topic: currentPeer, 
      content: text 
    });
    
    // Add message to UI optimistically
    addMessage({
      content: text,
      sender: 'me',
      timestamp: Date.now(),
      status: 'sent'
    });
    
    messageInput.value = '';
  } catch (err) {
    console.error('Send failed:', err);
  }
}

function addMessage(msg) {
  const div = document.createElement('div');
  div.className = `message ${msg.sender === 'me' ? 'sent' : 'received'}`;
  
  const text = document.createElement('div');
  text.className = 'text';
  text.textContent = msg.content;
  
  const meta = document.createElement('div');
  meta.className = 'meta';
  const time = new Date(msg.timestamp).toLocaleTimeString('en-US', { 
    hour: '2-digit', 
    minute: '2-digit' 
  });
  meta.innerHTML = msg.sender === 'me' 
    ? `${time} <span class="tick">✓✓</span>`
    : time;
  
  div.appendChild(text);
  div.appendChild(meta);
  
  // Remove empty state
  const emptyChat = messages.querySelector('.empty-chat');
  if (emptyChat) emptyChat.remove();
  
  messages.appendChild(div);
  messages.scrollTop = messages.scrollHeight;
}

function addChatItem(peer) {
  console.log('📝 addChatItem called with:', peer);
  const item = document.createElement('div');
  item.className = 'chat-item';
  item.dataset.peerId = peer.id;
  
  item.innerHTML = `
    <div class="name">${peer.name || peer.id.slice(0, 8)}...</div>
    <div class="preview">Connected</div>
  `;
  
  item.addEventListener('click', () => selectChat(peer.id));
  
  const emptyState = chatList.querySelector('.empty-state');
  if (emptyState) {
    console.log('  Removing empty state');
    emptyState.remove();
  }
  
  console.log('  Appending chat item to list');
  chatList.appendChild(item);
  peers.set(peer.id, peer);
  console.log('  ✅ Peer added, total peers:', peers.size);
}

function selectChat(peerId) {
  currentPeer = peerId;
  
  // Update UI
  document.querySelectorAll('.chat-item').forEach(item => {
    item.classList.toggle('active', item.dataset.peerId === peerId);
  });
  
  const peer = peers.get(peerId);
  peerInfo.classList.remove('hidden');
  peerName.textContent = peer.name || peerId.slice(0, 16) + '...';
  
  messageInput.disabled = false;
  sendBtn.disabled = false;
  messageInput.focus();
  
  // Clear messages (in real app, load chat history)
  messages.innerHTML = '';
}

// Listen for backend events
console.log('🔧 Setting up event listeners...');

listen('peer_connected', (event) => {
  console.log('🎉 peer_connected event received:', event.payload);
  alert('Peer connected: ' + event.payload.id); // Debug alert
  try {
    addChatItem(event.payload);
    console.log('✅ Chat item added to list');
  } catch (err) {
    console.error('❌ Failed to add chat item:', err);
  }
}).then(() => {
  console.log('✅ peer_connected listener registered');
}).catch(err => {
  console.error('❌ Failed to register peer_connected listener:', err);
});

listen('message_received', (event) => {
  console.log('Message received:', event.payload);
  const msg = event.payload;
  
  if (msg.sender === currentPeer) {
    addMessage({
      content: msg.content,
      sender: msg.sender,
      timestamp: msg.timestamp,
    });
  }
});

listen('handshake_completed', (event) => {
  console.log('Handshake completed:', event.payload);
  const peerId = event.payload.peer_id;
  const peer = peers.get(peerId);
  if (peer) {
    peer.quantumSafe = true;
  }
});

// Initialize - Start node on app launch
(async function init() {
  console.log('🚀 Initializing UMBRA...');
  
  try {
    const peerId = await invoke('start_node');
    myPeerId = peerId;
    myPeerIdEl.textContent = peerId;
    copyIdBtn.style.display = 'block';
    console.log('✅ Node started:', peerId);
    
    // Get listening addresses
    setTimeout(async () => {
      try {
        const addrs = await invoke('get_listen_addrs');
        console.log('📡 Listening on:', addrs);
        
        if (addrs && addrs.length > 0) {
          // Find the best address (prefer non-loopback IPv4)
          let bestAddr = addrs.find(a => a.includes('/ip4/') && !a.includes('127.0.0.1'));
          if (!bestAddr) bestAddr = addrs[0];
          
          // Append peer ID if not already there
          let fullAddr = bestAddr;
          if (!fullAddr.includes('/p2p/')) {
            fullAddr = `${bestAddr}/p2p/${peerId}`;
          }
          
          fullMultiaddrEl.textContent = fullAddr;
          listenAddrsEl.style.display = 'block';
          console.log('✅ Share this address:', fullAddr);
        }
      } catch (err) {
        console.error('Failed to get listen addresses:', err);
      }
    }, 1000); // Wait a bit for node to start listening
    
  } catch (err) {
    console.error('❌ Failed to start node:', err);
    myPeerIdEl.textContent = 'Error: ' + err;
    myPeerIdEl.style.color = '#ef4444';
  }
})();
