import React, { useEffect, useRef } from "react";
import MovesHistory from '../components/moves';


function GameGoal({auth, userGoals}) {
  const user_goal = userGoals.get(auth.id);

  return (
        <div className='w-28 space-y-4 overflow-auto'>
          <div className='bg-green-500 p-2 rounded-xl text-sm'>
            <p class="text-white">Your goal is : {user_goal}</p>
          </div>
        </div>
  )
}

function InitialValue() {

}

export default function GameBoard({auth, moves , userGoals, users}) {
  return (
      <div className='flex w-full justify-between'>
        <MovesHistory moves={moves} auth={auth} users={users} />
        <GameGoal auth={auth} userGoals={userGoals} />
      </div>
  )
}
