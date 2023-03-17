import React, { useState, useEffect } from "react";

function Card({onSelect, card, index}) {
    console.log(card);
    return (
        <div
        onClick={() => onSelect(value)}
        className='w-12 m-2 p-2 border-2 bg-blue-500 rounded-xl cursor-pointer  border-blue-300'>
          <p className='text-white'>{card.operator}{card.term}</p>
        </div>
    )
}



export default function cardHand({useCard, cards}) {
  const [data, setData] = useState([])
  const [isLoading, setLoading] = useState(false)

  return(
    <div className='w-full flex justify-center'>
        {isLoading && <p>Loading cards.</p>}
        {(cards.length == 0 && !isLoading) && <p>No cards available.</p>}
        {
            cards.map((item, index) => {
              return <Card
                onSelect={(idx) => useCard(item)}
                card={item}
                index={index}
              />
            })
        }
    </div>
  )
}
