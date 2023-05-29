import { useState, useEffect } from "react";

const useWebSocket = () => {
  const [socket, setSocket] = useState(null);

  useEffect(() => {
    const ws = new WebSocket("ws://localhost:8080/ws/");
    setSocket(ws);

    return () => {
      ws.close();
    };
  }, []);

  return socket;
};

export default useWebSocket;
