import React, { useEffect, useRef } from "react";
import Avatar from "./avatar"

function Move({ isAuthor, value, username }) {
    if (isAuthor) {
        return (
            <div className='w-full flex justify-end'>
                <div className='flex gap-3 justify-end'>
                    <div className='bg-violet-500 p-3 text-sm rounded-xl rounded-br-none'>
                        <p className='text-white'>{value}</p>
                    </div>
                </div>
            </div>
        )
    }

    return (
        <div className='flex gap-3 w-full'>
            <div className='bg-gray-200 p-3 text-sm rounded-xl rounded-bl-none'>
                <p>{value}</p>
            </div>
        </div>
    )
}

export default function MovesHistory({ moves, auth, users }) {
    const ref = useRef(null);

    useEffect(() => {
        ref.current?.scrollTo(0, ref.current.scrollHeight)
    }, [moves]);

    return (
        <div className='w-28 border-r-1  p-4 space-y-4 overflow-auto' ref={ref}>
            {
                moves.map(item => {
                    return <Move
                        isAuthor={item.user_id === auth.id}
                        value={item.value}
                        username={users.get(item.user_id)}
                        key={item.id} />
                })
            }
        </div>
    )
}
