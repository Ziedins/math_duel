import { useEffect, useRef } from "react";

export default function useWebsocket(onMove) {
    const ws = useRef(null);

    useEffect(() => {
        if (ws.current !== null) return;
        const wsUri = 'ws://localhost:8080/ws';
        ws.current = new WebSocket(wsUri);
        ws.current.onopen = () => console.log("ws opened");
        ws.current.onclose = () => console.log("ws closed");

        const wsCurrent = ws.current;
        return () => {
            wsCurrent.close();
        };
    }, []);

    useEffect(() => {
        if (!ws.current) return;
        ws.current.onmessage = e => {
            onMove(e.data)
        };
    }, []);

    const sendMove = (move) => {
        if (!ws.current) return;
        ws.current.send(move);
    }

    return sendMove;
}
