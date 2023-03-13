import { useEffect, useState } from "react";

const fetchCardData = async () => {
    const url = `http://localhost:8080/games/cards`;
    try {
        let resp = await fetch(url).then(res => res.json());
        return resp;
    } catch (e) {
        console.log(e);
    }
}

export default function useCards() {
    const [isLoading, setIsLoading] = useState(true);
    const [cards, setCards] = useState([]);

    const updateCards = (resp = []) => {
        setIsLoading(false);
        setCards(resp)
    }

    const fetchCards = () => {
        setIsLoading(true)
        fetchCardData().then(updateCards)
    }

    useEffect(() => fetchCards(), []);

    return [isLoading, cards, setCards, fetchCards];
}
