import "../styles/globals.css";
import type { AppProps } from "next/app";
import { ChakraProvider } from "@chakra-ui/react";
import SimpleSidebar from "../components/sidebar";

export default function App({ Component, pageProps }: AppProps) {
    return (
        <ChakraProvider>
            <SimpleSidebar>
                <Component {...pageProps} />
            </SimpleSidebar>
        </ChakraProvider>
    );
}
