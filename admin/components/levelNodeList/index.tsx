import { Box, Flex, Text } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import LevelNode, { LevelNodeProps } from "../levelNode";
import { useEffect, useState } from "react";

/*
    json format:
    {
        ...,
        content_levels: [
            [
                0, // index of level
                [ // nodes
                    { // node
                        index: 0, // removed before sending request
                        content_type: "text",
                        data: "Test",
                        onEdit: () => {}, // removed before sending request
                        onDelete: () => {}, // removed before sending request
                    },
                    { // node
                        index: 1, // removed before sending request
                            content_type: "image",
                        data: "base64",
                        onEdit: () => {}, // removed before sending request
                        onDelete: () => {}, // removed before sending request
                    }
                ]
            ],
            [
                1, // index of level
                [ // nodes
                    { // node
                        index: 0, // removed before sending request
                        content_type: "text",
                        data: "Test",
                        onEdit: () => {}, // removed before sending request
                        onDelete: () => {}, // removed before sending request
                    },
                ]
            ]
        ]
    }

    Algorithm for displaying levels:
    1. Sort content_levels by index of level
    2. Display each level
        
*/
export interface LevelNodeListProps {
    nodeLevels: LevelNodeProps[];

    nodeOnEdit?: () => void;
    nodeOnDelete?: () => void;
    // add Symbol.iterator to LevelNodeListProps
    [Symbol.iterator]?: () => Iterator<LevelNodeProps>;
}

// List of levelNode component
export default function LevelNodeList(props: LevelNodeListProps) {
    const [nodeLevels, setNodeLevels] = useState<LevelNodeProps[]>([]);

    useEffect(() => {
        setNodeLevels(props.nodeLevels);
    }, []);

    console.log(nodeLevels);

    return (
        <Flex flexDirection={"column"}>
            {/* sort by first element in level and then display */}
            {nodeLevels &&
                nodeLevels
                    .sort((a: any, b: any) => a[0] - b[0])
                    .map((level: any, index: any) => {
                        return (
                            <LevelNode
                                key={index}
                                index={level[0]}
                                nodes={level[1]}
                            />
                        );
                    })}
        </Flex>
    );
}
