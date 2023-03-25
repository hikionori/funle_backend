import { Button, Center, Text } from "@chakra-ui/react";
import router from "next/router";
import { FaPlus } from "react-icons/fa";

export interface BottomFloatingButtonProps {
    onClick: () => void;
    icon: React.ReactNode;
    text?: string;
}

export default function BottomFloatingButton(props: BottomFloatingButtonProps) {
    if (props.text) {
        return (
            // Button with text and icon
            <Button
                position={"fixed"}
                bgColor={"white"}
                _hover={{
                    bgColor: "black",
                    color: "white",
                    transition: "50ms linear",
                }}
                borderRadius="25"
                bottom="40px"
                right={"40px"}
                height="60px"
                width="120px"
                zIndex={2}
                onClick={props.onClick}
            >
                <Center>
                    {props.icon}
                    <Text marginLeft={"5px"} fontWeight="bold">{props.text}</Text>
                </Center>
            </Button>
        );
    }
    return (
        <Button
            position={"fixed"}
            bgColor={"white"}
            _hover={{
                bgColor: "black",
                color: "white",
                transition: "50ms linear",
            }}
            borderRadius="full"
            bottom="40px"
            right={"40px"}
            height="70px"
            width="70px"
            zIndex={2}
            onClick={props.onClick}
        >
            {props.icon}
        </Button>
    );
}
