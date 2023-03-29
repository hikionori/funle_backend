import {
    Box,
    Divider,
    Flex,
    Icon,
    Spacer,
    Stack,
    Text,
} from "@chakra-ui/react";
import TutorialCard, {TutorialCardProps} from "../card/tutorialCard";
import { FaChevronUp, FaChevronDown } from "react-icons/fa";
import React from "react";

interface TutorialCardListProps {
    cards: TutorialCardProps[];
    theme: string;
}

export default function TutorialCardList(props: TutorialCardListProps) {
    const { cards, theme } = props;

    const [isFolded, setIsFolded] = React.useState(false);

    const unfoldIcon = <FaChevronDown size={"25"} />;
    const foldIcon = <FaChevronUp size={"25"} />;

    return (
        <Box
            bgColor={"whiteAlpha.800"}
            borderRadius="xl"
            padding={"5"}
            marginBottom="5"
        >
            <Flex>
                {/* Dropdown element */}
                <Text paddingLeft={"5"}>{theme}</Text>
                <Spacer />
                <Box
                    bgColor={"blackAlpha.200"}
                    boxSize="36px"
                    borderRadius="xl"
                    marginRight={"10"}
                    justifyContent={"center"}
                    alignItems={"center"}
                    display={"flex"}
                >
                    <Icon onClick={() => setIsFolded(!isFolded)}>
                        {isFolded ? unfoldIcon : foldIcon}
                    </Icon>
                </Box>
            </Flex>
            <Box display={!isFolded ? "contents" : "none"}>
                {!isFolded &&
                    cards.map((card) => (
                        <Stack
                            paddingLeft={"10"}
                            // paddingTop="5"
                            paddingRight={"10"}
                            direction={"row"}
                        >
                            <Divider
                                orientation="vertical"
                                height={"20"}
                                marginRight="5"
                                width={"1px"}
                            />
                            <Box flex={"1"}>
                                <TutorialCard
                                    key={card.id}
                                    id={card.id}
                                    title={card.title}
                                    onClick={card.onClick}
                                    onDelete={card.onDelete}
                                />
                            </Box>
                        </Stack>
                    ))}
            </Box>
        </Box>
    );
}
