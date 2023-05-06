import {
    Box,
} from "@chakra-ui/react";
import CourceCard, { CourceCardProps } from "../card/courceCard";
import React from "react";

interface CourceCardListProps {
    cards: CourceCardProps[];
}

export default function CourceCardList(props: CourceCardListProps) {
    const cards = props.cards;

    return (
        <Box
            bgColor={"whiteAlpha.800"}
            borderRadius="xl"
            padding={"5"}
            marginBottom="5"
        >
            <Box>
                {cards.map((card) => (
                        <Box flex={"1"}>
                            <CourceCard
                                key={card.id}
                                id={card.id}
                                title={card.title}
                                onClick={card.onClick}
                                onDelete={card.onDelete}
                            />
                        </Box>
                ))}
            </Box>
        </Box>
    );
}
