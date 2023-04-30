import { Text } from "@chakra-ui/react";
import { useRouter } from "next/router";


export default function EditCource() {

    const router = useRouter();
    const id = router.query.id;

    return (
        <Text>
            {id}
        </Text>
    )
}