import React, { useEffect, useRef } from "react";
import Avatar from "./avatar"

function Move({ isAuthor, operator, term}) {
    if (isAuthor) {
        return (
            <div className='w-full flex justify-end'>
                <div className='flex gap-3 justify-end'>
                    <div className='bg-violet-500 p-3 text-sm rounded-xl rounded-br-none'>
                        <p className='text-white'>{operator}{term}</p>
                    </div>
                </div>
            </div>
        )
    }

    return (
        <div className='flex gap-3 w-full'>
            <div className='bg-gray-200 p-3 text-sm rounded-xl rounded-bl-none'>
                <p>{operator}{term}</p>
            </div>
        </div>
    )
}

export default function MovesHistory({ moves, auth }) {
    const ref = useRef(null);
    console.log(moves);
    useEffect(() => {
        ref.current?.scrollTo(0, ref.current.scrollHeight)
    }, [moves]);
    return (
        <div className='w-28 border-r  p-4 space-y-4 overflow-auto' ref={ref}>
            {
                moves.map(item => {
                    return <Move
                        isAuthor={item.user_id === auth.id}
                        operator={item.operator}
                        term = {item.term}
                        key={item.id} />
                })
            }
        </div>
    )
}
