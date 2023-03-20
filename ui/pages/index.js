import Head from 'next/head'
import React, { useEffect, useState } from 'react'
import Avatar from '../components/avatar'
import GameList from '../components/game'
import GameBoard from '../components/gameBoard'
import Login from '../components/login'
import useMoves from '../libs/useMove'
import useLocalStorage from '../libs/useLocalStorage'
import useWebsocket from '../libs/useWebsocket'
import useCards from '../libs/useCard'

export default function Home() {
  const [game, setSelectedGame] = useState(null);
  const [showLogIn, setShowLogIn] = useState(false);
  const [auth, setAuthUser] = useLocalStorage("user", false);
  const [isLoading, moves, setMoves, fetchMoves] = useMoves("");
  const [isCardsLoading, cards, setCards, fetchCards] = useCards("");

  const handleMove = (move, userId) => {
    setMoves(prev => {
      const item = { operator: move.operator, term: move.term, user_id: userId };
      return [...prev, item];
    })
  }

  const onMove = (data) => {
    try {
      let moveData = JSON.parse(data);
      handleMove(moveData, moveData.user_id);
      return;
    } catch (e) {
      console.log(e);
    }
  }

  const sendMove = useWebsocket(onMove)


  const makeMove = (move) => {
    if (!move) {
      return;
    }

    if (!game.id) {
      alert("Please select a game!");
      return;
    }

    const data = {
      id: 0,
      math_duel_type: "MOVE",
      operator: move.operator,
      term: move.term,
      game_id: game.id,
      user_id: auth.id
    }

    sendMove(JSON.stringify(data));
    handleMove(move, auth.id);
  }

  const updateGame = (data) => {
    if (!data.id) return;
    fetchMoves(data.id)
    fetchCards(data.id)
    setSelectedGame(data)
  }

  const signOut = () => {
    window.localStorage.removeItem("user");
    setAuthUser(false);
  }

  useEffect(() => setShowLogIn(!auth), [auth])
  return (
    <div>
      <Head>
        <title>Math Duel game</title>
        <meta name="description" content="Math Duel game" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Login show={showLogIn} setAuth={setAuthUser} />
      <div className={`${!auth && 'hidden'} bg-gradient-to-b from-orange-400 to-rose-400 h-screen p-12`}>
        <main className='flex w-full max-w-[1020px] h-[700px] mx-auto bg-[#FAF9FE] rounded-[25px] backdrop-opacity-30 opacity-95'>
          <aside className='bg-[#F0EEF5] w-[325px] h-[700px] rounded-l-[25px] p-4 overflow-auto relative'>
            <GameList onGameChange={updateGame}  userId={auth.id} />
            <button onClick={signOut} className='text-xs w-full max-w-[295px] p-3 rounded-[10px] bg-violet-200 font-semibold text-violet-600 text-center absolute bottom-4'>LOG OUT</button>
          </aside>
          {game?.id && (<section className='rounded-r-[25px] w-full max-w-[690px] grid grid-rows-[80px_minmax(450px,_1fr)]'>
            <div className='rounded-tr-[25px] w-ful'>
              <div className='flex gap-3 p-3 items-center'>
                <Avatar color='rgb(245 158 11)'>{game.users.get_target_user(auth.id)}</Avatar>
                <div>
                  <p className='font-semibold text-gray-600 text-base'>You vs {game.users.get_target_user(auth.id)}</p>
                </div>
              </div>
              <hr className='bg-[#F0EEF5]' />
            </div>
            {(isLoading && game.id) && <p className="px-4 text-slate-500">Loading game...</p>}
            <GameBoard auth={auth} moves={moves} game={game} cards={cards} useCard={makeMove}/>
          </section>)}
        </main>
      </div>
    </div>
  )
}
