import { useState, useEffect } from "react";
import "@chatscope/chat-ui-kit-styles/dist/default/styles.min.css";
import {
  MainContainer,
  ChatContainer,
  MessageList,
  Message,
  MessageInput,
  TypingIndicator,
} from "@chatscope/chat-ui-kit-react";
import "./App.css";

function App() {
  const [messages, setMessages] = useState([
    {
      message: "Hello, I'm a Chatty Llama! Ask me anything!",
      sentTime: "just now",
      sender: "ChattyLlama",
    },
  ]);
  const [isTyping, setIsTyping] = useState(false);
  const [ws, setWs] = useState(null);

  useEffect(() => {
    const createWebSocketConnection = (port) => {
      const wsURL = `ws://localhost:${port}/ws/`;
      console.log("Attempting connection to: ", wsURL);
      const websocket = new WebSocket(wsURL);

      websocket.onopen = () => {
        console.log("Connected to the WebSocket server on port", port);
        setWs(websocket);
      };

      websocket.onerror = (error) => {
        console.log(`WebSocket Error on port ${port}:`, error);
        // If connection fails on port 80, try connecting on port 8080
        if (port === 80) {
          console.log("Attempting to connect on port 8080...");
          createWebSocketConnection(8080);
        }
      };

      websocket.onmessage = (event) => {
        const botResponse = event.data; // Assuming server sends raw string as response

        setMessages((prev) => [
          ...prev,
          {
            message: botResponse,
            sender: "ChattyLlama",
          },
        ]);
        setIsTyping(false);
      };

      return websocket;
    };

    const wsInstance = createWebSocketConnection(80); // Start with port 80

    return () => {
      if (wsInstance) wsInstance.close();
    };
  }, []);

  const handleSend = (message) => {
    const newMessage = {
      message,
      direction: "outgoing",
      sender: "user",
    };

    setMessages((prev) => [...prev, newMessage]);

    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(message);
      setIsTyping(true); // Assume bot is typing after we send a message.
    }
  };

  return (
    <div className="App app-background">
      <div style={{ position: "relative", height: "800px", width: "700px" }}>
        <MainContainer>
          <ChatContainer>
            <MessageList
              scrollBehavior="smooth"
              typingIndicator={
                isTyping ? (
                  <TypingIndicator content="Chatty Llama is typing" />
                ) : null
              }
            >
              {messages.map((message, i) => {
                return <Message key={i} model={message} />;
              })}
            </MessageList>
            <MessageInput placeholder="Type message here" onSend={handleSend} />
          </ChatContainer>
        </MainContainer>
      </div>
    </div>
  );
}

export default App;
