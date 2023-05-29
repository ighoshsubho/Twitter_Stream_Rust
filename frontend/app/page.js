"use client"
import { useState } from "react";
import useWebSocket from "@/components/ws";

export default function Home() {
  const [tweets, setTweets] = useState([]);
  const [keyword, setKeyword] = useState("");
  const socket = useWebSocket();

  const handleKeywordChange = (event) => {
    setKeyword(event.target.value);
  };

  const handleKeywordSubmit = (event) => {
    event.preventDefault();

    // Send keyword to the server
    socket.send(keyword);
  };

  // Listen for incoming tweets
  if (socket) {
    socket.onmessage = (event) => {
      const tweet = JSON.parse(event.data);
      setTweets((prevTweets) => [...prevTweets, tweet]);
    };
  }

  return (
    <div className="container mx-auto">
      <h1 className="text-2xl font-bold my-4">Twitter Streaming Example</h1>

      <form onSubmit={handleKeywordSubmit}>
        <input 
          type="text" 
          id="first_name" 
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" 
          placeholder="Enter keyword"
          onChange={handleKeywordChange}
          value={keyword}
          />
        <button
          type="submit"
          className="bg-blue-500 text-white py-2 px-4 rounded"
        >
          Start Streaming
        </button>
      </form>

      <div className="mt-4">
        {tweets.map((tweet) => (
          <div
            key={tweet.id}
            className="border border-gray-300 rounded p-4 my-2"
          >
            <h3 className="font-bold text-xl">{tweet.user.name}</h3>
            <p className="text-gray-600">{tweet.text}</p>
          </div>
        ))}
      </div>
    </div>
  );
};
