import { useEffect, useState } from "react";

const fetchGameData = async (game_id) => {
    if (!game_id) return;
    const url = `http://localhost:8080/moves/${game_id}`;
    try {
        let resp = await fetch(url).then(res => res.json());
        return resp;
    } catch (e) {
        console.log(e);
    }
}

export default function useMoves(game_id) {
    const [isLoading, setIsLoading] = useState(true);
    const [moves, setMoves] = useState([]);

    const updateMoves = (resp = []) => {
        setIsLoading(false);

        console.log(resp);
        setMoves(resp)
    }

    const fetchMoves = (id) => {
        setIsLoading(true)
        fetchGameData(id).then(updateMoves)
    }

    useEffect(() => fetchMoves(game_id), []);

    return [isLoading, moves, setMoves, fetchMoves];
}
