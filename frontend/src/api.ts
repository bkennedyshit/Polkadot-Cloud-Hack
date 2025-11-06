import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export interface ReputationProfile {
  totalScore: number;
  reviewCount: number;
  categoryScores: {
    communication: number;
    reliability: number;
    quality: number;
    professionalism: number;
  };
  active: boolean;
  stakedAmount: string;
  owner: string;
}

export interface Rating {
  from: string;
  to: string;
  score: number;
  categoryRatings: {
    communication: number;
    reliability: number;
    quality: number;
    professionalism: number;
  };
  timestamp: number;
  reviewHash: string;
}

export interface UserStats {
  address: string;
  averageScore: number;
  totalReviews: number;
  categoryAverages: {
    communication: number;
    reliability: number;
    quality: number;
    professionalism: number;
  };
  stakedAmount: string;
  isActive: boolean;
}

let apiInstance: ApiPromise | null = null;

export async function initializeApi(wsEndpoint: string = 'ws://localhost:9944'): Promise<ApiPromise> {
  if (apiInstance) {
    return apiInstance;
  }

  const wsProvider = new WsProvider(wsEndpoint);
  apiInstance = await ApiPromise.create({ provider: wsProvider });
  return apiInstance;
}

export async function getApi(): Promise<ApiPromise> {
  if (!apiInstance) {
    return initializeApi();
  }
  return apiInstance;
}

export async function connectWallet() {
  const extensions = await web3Enable('ReputeChain');
  if (extensions.length === 0) {
    throw new Error('No Polkadot wallet extension found');
  }

  const accounts = await web3Accounts();
  return accounts;
}

export async function createProfile(address: string) {
  const api = await getApi();
  const injector = await web3FromAddress(address);
  
  const tx = api.tx.reputation.createProfile();
  const hash = await tx.signAndSend(address, { signer: injector.signer });
  
  return hash;
}

export async function submitRating(
  fromAddress: string,
  toAddress: string,
  score: number,
  communication: number,
  reliability: number,
  quality: number,
  professionalism: number,
  reviewHash: string = '0x' + '0'.repeat(64)
) {
  const api = await getApi();
  const injector = await web3FromAddress(fromAddress);
  
  const tx = api.tx.reputation.submitRating(
    toAddress,
    score,
    communication,
    reliability,
    quality,
    professionalism,
    reviewHash
  );
  
  const hash = await tx.signAndSend(fromAddress, { signer: injector.signer });
  return hash;
}

export async function stakeReputation(address: string, amount: string) {
  const api = await getApi();
  const injector = await web3FromAddress(address);
  
  const tx = api.tx.reputation.stakeReputation(amount);
  const hash = await tx.signAndSend(address, { signer: injector.signer });
  
  return hash;
}

export async function getUserReputation(address: string): Promise<ReputationProfile | null> {
  const api = await getApi();
  const profile = await api.query.reputation.userReputation(address);
  
  if (!profile.isEmpty) {
    const data = profile.toJSON() as any;
    return {
      totalScore: data.totalScore,
      reviewCount: data.reviewCount,
      categoryScores: data.categoryScores,
      active: data.active,
      stakedAmount: data.stakedAmount.toString(),
      owner: data.owner,
    };
  }
  
  return null;
}

export async function getUserStats(address: string): Promise<UserStats> {
  const profile = await getUserReputation(address);
  
  if (!profile) {
    return {
      address,
      averageScore: 0,
      totalReviews: 0,
      categoryAverages: {
        communication: 0,
        reliability: 0,
        quality: 0,
        professionalism: 0,
      },
      stakedAmount: '0',
      isActive: false,
    };
  }

  const avgScore = profile.reviewCount > 0 ? profile.totalScore / profile.reviewCount : 0;
  const categoryAverages = {
    communication: profile.reviewCount > 0 ? profile.categoryScores.communication / profile.reviewCount : 0,
    reliability: profile.reviewCount > 0 ? profile.categoryScores.reliability / profile.reviewCount : 0,
    quality: profile.reviewCount > 0 ? profile.categoryScores.quality / profile.reviewCount : 0,
    professionalism: profile.reviewCount > 0 ? profile.categoryScores.professionalism / profile.reviewCount : 0,
  };

  return {
    address,
    averageScore: Math.round(avgScore * 100) / 100,
    totalReviews: profile.reviewCount,
    categoryAverages,
    stakedAmount: profile.stakedAmount,
    isActive: profile.active,
  };
}

export async function getTopUsers(limit: number = 10): Promise<UserStats[]> {
  // This would need to be implemented with a more sophisticated query
  // For now, returning empty array - would need indexing service in production
  return [];
}

export async function deactivateProfile(address: string) {
  const api = await getApi();
  const injector = await web3FromAddress(address);
  
  const tx = api.tx.reputation.deactivateProfile();
  const hash = await tx.signAndSend(address, { signer: injector.signer });
  
  return hash;
}
