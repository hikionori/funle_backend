import { Box } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import LevelNode, {LevelNodeProps} from "../levelNode";



/*
    json format:
    {
        ...,
        content_levels: [
            [
                0,
                [
                    {
                        index: 0, // removed before sending request
                        content_type: "text",
                        data: "Test",
                        onEdit: () => {}, // removed before sending request
                        onDelete: () => {}, // removed before sending request
                    },
                    {
                        index: 1, // removed before sending request
                        content_type: "image",
                        data: "base64",
                        onEdit: () => {}, // removed before sending request
                        onDelete: () => {}, // removed before sending request
                    }
                ]
            ],
            [
                1,
                [
                    {
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
        
*/
export interface LevelNodeListProps {
    nodeLevels: LevelNodeProps[];

    nodeOnEdit?: () => void;
    nodeOnDelete?: () => void;
    // add Symbol.iterator to LevelNodeListProps
    [Symbol.iterator]: () => Iterator<LevelNodeProps>;
}

// List of levelNode component
export default function LevelNodeList(props: LevelNodeListProps) {
    return (
        <Box></Box>
    );
}