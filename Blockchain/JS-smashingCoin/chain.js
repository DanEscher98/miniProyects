const SHA256 = require('crypto-js/sha256');

class CryptoBlock {
	constructor(index, timestamp, data, precedingHash=" ") {
		// A unique number that tracks the position of every block
		this.index = index;
		// Keeps a record of the time of occurrence of each transaction
		this.timestamp = timestamp;
		// Provides data about the completed transactions
		this.data = data;
		// It points to the hash of the preceding block
		this.precedingHash = precedingHash;
		// 
		this.hash = this.computeHash();
		// A random value to preserve the difficulty
		this.nonce = 0;
	}
	
	computeHash() {
		return SHA256(
			this.index
			+ this.precedingHash
			+ this.timestamp
			+ JSON.stringify(this.data)
			+ this.nonce
		).toString();
	}
	
	// This algorithm prevents generatin invalid new blocks
	// easily or spamming the blockchain. The difficulty is
	// an integer number that puts that number of zeros at
	// the end. 
	proofOfWork(difficulty) {
		while (
			this.hash.substring(0, difficulty) !== Array(difficulty + 1).join("0")
		) {
			this.nonce++;
			this.hash = this.computeHash();
		}
	}
}

class CryptoBlockchain {
	constructor() {
		this.blockchain = [this.startGenesisBlock()];
		this.difficulty = 4;
	}

	startGenesisBlock() {
		return new CryptoBlock(0, "14/Oct/2021", "Initial Block in the Chain", "0");
	}

	obtainLatestBlock() {
		return this.blockchain[this.blockchain.length - 1];
	}

	addNewBlock(newBlock) {
		newBlock.precedingHash = this.obtainLatestBlock().hash;
		// newBlock.hash = newBlock.computeHash();
		newBlock.proofOfWork(this.difficulty);
		this.blockchain.push(newBlock);
	}

	checkChainValidity() {
		for (let i=1; i < this.blockchain.length; i++) {
			const currentBlock = this.blockchain[i];
			const precedingBlock = this.blockchain[i-1];
			
			// Check that the stored hash is the real hash
			if (currentBlock.hash !== currentBlock.computeHash()) {
				return false;
			}
			// 
			if (currentBlock.precedingHash !== precedingBlock.hash) {
				return false;
			}
		}
		return true;
	}
}

let smashingCoin = new CryptoBlockchain();

//console.log("smashingcoin mining in progress.....");
smashingCoin.addNewBlock(
	new CryptoBlock(1, "15/10/2021", {
		sender: "Daniel Sanchez",
		recipient: "Cosima Mielke",
		quantity: 50
	})
);

smashingCoin.addNewBlock(
	new CryptoBlock(2, "20/10/2021", {
		sender: "Vitaly Friedman",
		recipient: "Ricardo Jimenez",
		quantity: 100
	})
);

console.log(JSON.stringify(smashingCoin, null, 4));
