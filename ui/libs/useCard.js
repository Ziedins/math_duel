import { useEffect, useState } from "react";

const fetchCardData = async (game_id) => {
    if (!game_id) return;
    const url = `http://localhost:8080/cards/game/${game_id}`;
    try {
        let resp = await fetch(url).then(res => res.json());
        return resp;
    } catch (e) {
        console.log(e);
    }
}

export default function useCards(game_id) {
    const [isLoading, setIsLoading] = useState(true);
    const [cards, setCards] = useState([]);

    const updateCards = (resp = []) => {
        setIsLoading(false);
        setCards(resp)
    }

    const fetchCards = (id) => {
        setIsLoading(true)
        fetchCardData(id).then(updateCards)
    }

    useEffect(() => fetchCards(game_id), []);

    return [isLoading, cards, setCards, fetchCards];
}
