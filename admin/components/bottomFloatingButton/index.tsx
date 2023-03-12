import { Button } from "@chakra-ui/react";
import router from "next/router";
import { FaPlus } from "react-icons/fa";

export interface BottomFloatingButtonProps {
    onClick: () => void;
    icon: React.ReactNode;
}

export default function BottomFloatingButton(props: BottomFloatingButtonProps) {
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