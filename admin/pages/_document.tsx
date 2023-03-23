import { Html, Head, Main, NextScript } from "next/document";
import SimpleSidebar from "../components/sidebar";

export default function Document() {
    return (
        <Html lang="en">
            <Head />
            <body style={{
                background: "#F5F5F5",
            }}>
                <Main />
                <NextScript />
            </body>
        </Html>
    );
}
