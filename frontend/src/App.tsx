import { useEffect, useRef, useState, useCallback } from "react";
import "./App.css";

interface Block {
  index: number;
  timestamp: number;
  sender: string;
  receiver: string;
  data: string;
  previous_hash: string;
  hash: string;
}

export default function App() {
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [sender, setSender] = useState("");
  const [receiver, setReceiver] = useState("");
  const [data, setData] = useState("");
  const [selectedBlock, setSelectedBlock] = useState<Block | null>(null);
  const blockchainRef = useRef<HTMLDivElement>(null);

  const fetchBlocks = useCallback(async () => {
    try {
      const response = await fetch("http://localhost:8080/blocks");
      if (!response.ok) throw new Error("Failed to fetch blocks");
      const data = await response.json();
      setBlocks(data);
    } catch (error) {
      console.error("Error fetching blocks:", error);
    }
  }, []);

  useEffect(() => {
    fetchBlocks();
  }, [fetchBlocks]);

  async function addBlock() {
    const trimmedSender = sender.trim();
    const trimmedReceiver = receiver.trim();
    const trimmedData = data.trim();

    if (!trimmedSender || !trimmedReceiver || !trimmedData) {
      alert("All fields are required!");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/add_block", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ sender: trimmedSender, receiver: trimmedReceiver, data: trimmedData }),
      });

      if (response.ok) {
        fetchBlocks();
        setSender("");
        setReceiver("");
        setData("");
      } else {
        alert("Failed to add block");
      }
    } catch (error) {
      console.error("Error adding block:", error);
    }
  }

  return (
    <div className="app">
      <header className="header">
        <h1>Add Block</h1>
        <div className="input-container">
          <input type="text" placeholder="Sender" value={sender} onChange={(e) => setSender(e.target.value)} />
          <input type="text" placeholder="Receiver" value={receiver} onChange={(e) => setReceiver(e.target.value)} />
          <input type="text" placeholder="Data" value={data} onChange={(e) => setData(e.target.value)} />
          <button onClick={addBlock}>Add Block</button>
        </div>
      </header>

      <main className="main-content">
        <h2>Blockchain Explorer</h2>
        <div className="blockchain-container" ref={blockchainRef}>
          {blocks.map((block, index) => (
            <div key={block.index} className="block-wrapper">
              <div className="block-circle" onClick={() => setSelectedBlock(block)}>
                <h3>Block #{block.index}</h3>
              </div>
              {index < blocks.length - 1 && <div className="block-link"></div>}
            </div>
          ))}
        </div>
      </main>

      {selectedBlock && (
        <div className="modal-overlay" onClick={() => setSelectedBlock(null)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <button className="close-button" onClick={() => setSelectedBlock(null)}>✖</button>
            <div className="modal-header">BLOCK #{selectedBlock.index}</div>
            <div className="modal-grid">
              <div className="detail-box"><strong>ID</strong> {selectedBlock.index}</div>
              <div className="detail-box"><strong>Timestamp</strong> {new Date(selectedBlock.timestamp * 1000).toLocaleString()}</div>
              <div className="detail-box"><strong>Sender</strong> {selectedBlock.sender}</div>
              <div className="detail-box"><strong>Receiver</strong> {selectedBlock.receiver}</div>
              <div className="detail-box" style={{ gridColumn: "span 2" }}><strong>Data</strong> {selectedBlock.data}</div>
              <div className="detail-box" style={{ gridColumn: "span 2" }}>
                <strong>Hash</strong> <span style={{ wordBreak: "break-word" }}>{selectedBlock.hash}</span>
              </div>
              <div className="detail-box" style={{ gridColumn: "span 2" }}>
                <strong>Previous Hash</strong> <span style={{ wordBreak: "break-word" }}>{selectedBlock.previous_hash}</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
