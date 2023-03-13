import React, { useEffect, useRef } from "react";
import MovesHistory from '../components/moves';


function GoalNumber({auth, userGoals, game}) {
  const user_goal = userGoals.get(auth.id);

  return (
        <div className='w-28 p-4 space-y-4 overflow-auto'>
          <div className='bg-green-500 p-2 rounded-xl text-sm'>
            <p className="text-white">Your goal is : {user_goal}</p>
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

export default function GameBoard({auth, moves, game}) {
  const users = game.users;
  const userGoals = game.userGoals;
  const initialNumber = game.current_value;
  return (
      <div className='flex w-full justify-between'>
        <MovesHistory moves={moves} auth={auth} users={users} />
        <InitialNumber number={initialNumber} />
        <GoalNumber auth={auth} userGoals={userGoals} game={game} />  
      </div>
  )
}
