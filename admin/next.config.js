/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  serverRuntimeConfig: {
    // Will only be available on the server side
    mySecret: 'secret',
    host: '0.0.0.0',
  },
  publicRuntimeConfig: {
    // Will be available on both server and client
    staticFolder: '/static',
    host: '0.0.0.0',
  },
}

module.exports = nextConfig
