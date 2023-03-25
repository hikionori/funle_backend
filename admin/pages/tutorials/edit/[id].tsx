import Head from "next/head";
import { useRouter } from "next/router";
import { useEffect } from "react";

export default function EditTutorial() {

    const router = useRouter();
    const id = router.query.id;

    return (
        <>
         <Head>
                <title>Create new tutorial</title>
            </Head>
            <main>
                <h1>Tutorilas {id}</h1>
            </main>
        </>
    )
}