import React, { useState, useEffect } from "react";

function Card({onSelect, value, index}) {
    return (
        <div 
        onClick={() => onSelect(index, value)}
        className='w-12 m-2 p-2 border-2 bg-blue-500 rounded-xl cursor-pointer  border-blue-300'>
          <p className='text-white'>{value}</p>
        </div>
    )
}



export default function cardHand({cards}) {
  const [data, setData] = useState([])
  const [isLoading, setLoading] = useState(false)

  const onSelectedCard = (idx, item) => {
    console.log(idx, item);
  }

  return(
    <div className='w-full flex justify-center'>
        {isLoading && <p>Loading cards.</p>}
        {(cards.length == 0 && !isLoading) && <p>No cards available.</p>}
        {
            cards.map((item, index) => {
              return <Card
                onSelect={(idx) => onSelectedCard(idx, item)}
                value={item}
                index={index}
              />
            })
        }
    </div>
  )
}
