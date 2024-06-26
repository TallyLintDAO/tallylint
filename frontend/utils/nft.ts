import { getAllUserNFTs } from "@psychedelic/dab-js"
// readme: https://github.com/bitfinity-network/DAB-js?tab=readme-ov-file

export const getNFTCollections = async (principal: string) => {
  const collections = await getAllUserNFTs({
    user: principal,
  })
  console.log("NFT collections", collections)
}
