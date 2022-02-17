import React from 'react'
import './Deck.css'

export type DeckButtonProps = {
    text: string;
}

export function DeckButton({text}: DeckButtonProps) {
    return (
        <div className='deck-button'>
            {text}
        </div>
    )
}

export type DeckProps = {
    rows: number;
    cols: number;
}

export function Deck({rows, cols}: DeckProps) {
    return (
        <div className='grid grid-cols-5 grid-flow-row gap-2 place-content-center deck deck-original'>
            {
                [...Array(rows*cols).keys()].map(() => 
                    <DeckButton text="test"></DeckButton>
                )
            }
        </div>
    )
}
