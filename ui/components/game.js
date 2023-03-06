import React, { useState, useEffect } from "react";

async function getGames() {
    try {
        const url = "http://localhost:8080/games";
        let result = await fetch(url);
        return result.json();
    } catch (e) {
        console.log(e);
        return Promise.resolve(null);
    }
}

async function getGamesByUser(userId) {
    try {
        const url = "http://localhost:8080/games/user/" + userId;
        let result = await fetch(url);
        return result.json();
    } catch (e) {
        console.log(e);
        return Promise.resolve(null);
    }
}

function GameListItem({ onSelect, game, userId, index, selectedItem }) {
    const { users, created_at, last_message } = game;
    console.log(game);
    const active = index == selectedItem;
    const date = new Date(created_at);
    const ampm = date.getHours() >= 12 ? 'PM' : 'AM';
    const time = `${date.getHours()}:${date.getMinutes()} ${ampm}`
    const name = users?.filter(user => user.id != userId).map(user => user.username)[0];
    const user_a_name = users[0].username;
    const user_b_name = users[1].username;
    const game_name = game.name;

    return (
        <div
            onClick={() => onSelect(index, {})}
            className={`${active ? 'bg-[#FDF9F0] border border-[#DEAB6C]' : 'bg-[#FAF9FE] border border-[#FAF9FE]'} p-2 rounded-[10px] shadow-sm cursor-pointer`} >
            <div className='flex justify-between items-center gap-3'>
                <div className='flex gap-3 items-center w-full'>
                    <div className="w-full max-w-[150px]">
                        <h2 className='font-semibold text-sm text-gray-500'>{game_name}</h2>
                        <h3 className='font-semibold text-sm text-gray-700'>{user_a_name} vs {user_b_name}</h3>
                        <p className='font-light text-xs text-gray-600 truncate'>{last_message}</p>
                    </div>
                </div>
                <div className='text-gray-400 min-w-[55px]'>
                    <span className='text-xs'>{time}</span>
                </div>
            </div>
        </div>
    )
}

export default function GameList({ onChatChange, userId}) {
   
    if(userId == undefined) return;
    const [data, setData] = useState([])
    const [isLoading, setLoading] = useState(false)
    const [selectedItem, setSelectedItem] = useState(-1);
   
    useEffect(() => {
        setLoading(true)
            getGamesByUser(userId)
            .then((data) => {
                setData(data)
                setLoading(false)
            })
    }, [])

    const onSelectedChat = (idx, item) => {
        setSelectedItem(idx)
        let mapUsers = new Map();
        item.users.forEach(el => {
            mapUsers.set(el.id, el);
        });
        const users = {
            get: (id) => {
                return mapUsers.get(id).username;
            },
            get_target_user: (id) => {
                return item.users.filter(el => el.id != id).map(el => el.username).join("")
            }
        }
        onChatChange({ ...item.game, users })
    }

    return (
        <div className="overflow-hidden space-y-3">
            {isLoading && <p>Loading game list.</p>}
            {(data.length == 0 && !isLoading) && <p>No games yet.</p>}
            {
                data.map((item, index) => {
                    return <GameListItem
                        onSelect={(idx) => onSelectedChat(idx, item)}
                        game={{ ...item.game, users: item.users }}
                        index={index}
                        key={item.game.id}
                        userId={userId}
                        selectedItem={selectedItem} />
                })
            }
        </div>
    )
}
