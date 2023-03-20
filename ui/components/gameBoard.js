import React, { useEffect, useRef } from "react";
import MovesHistory from '../components/moves';
import CardHand from '../components/cards';


function GoalNumber({auth, userGoal, game}) {

  return (
        <div className='w-28 p-4 space-y-4 overflow-auto'>
          <div className='bg-green-500 p-2 rounded-xl text-sm'>
            <p className="text-white">Your goal is : {userGoal}</p>
          </div>
          <DebugPanel game={game} />
        </div>
  )
}


function InitialNumber({number}) {
  return (
      <div className='w-28 p-4 space-y-4 overflow-auto'>
        <div className='bg-gray-300 p-2 rounded-xl text-sm'>
            <p>The inital number is : {number}</p>
        </div>
      </div>
  )
}

function DebugPanel({game}) {
  return (
      <div className='bg-yellow-100 border-orange-300'>
        <p>Current value: {game.current_value}</p>
      </div>
  )
}

export default function GameBoard({auth, moves, game, cards, useCard}) {
    const users = game.users;
    const userGoal = game.userGoals.get(auth.id);
    const initialNumber = game.initial_value;
    const authIsPlayerA = game.user_a_id == auth.id;
    const isAuthTurn = game.authUser.isTurn;

    console.log(game);
    return (
        <div className='grid grid-rows-[minmax(450px,_1fr)_65px]'>

            <div className='flex w-full justify-between'>
                <MovesHistory moves={moves} auth={auth} users={users} />
                <InitialNumber number={initialNumber} />
                <GoalNumber auth={auth} userGoal={userGoal} game={game} />
            </div>

            <CardHand useCard={useCard} cards={cards} isAuthTurn={isAuthTurn} />

        </div>
    )
}
