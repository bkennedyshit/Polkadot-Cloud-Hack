import React, { useState, useEffect } from 'react';
import { Star, TrendingUp, Users, Award, Wallet } from 'lucide-react';
import { getUserStats, connectWallet, createProfile, submitRating } from './api';

interface DashboardProps {
  userAddress?: string;
}

export default function Dashboard({ userAddress }: DashboardProps) {
  const [stats, setStats] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [connected, setConnected] = useState(false);
  const [currentUser, setCurrentUser] = useState<string | null>(userAddress || null);
  const [ratingTarget, setRatingTarget] = useState('');
  const [ratingScore, setRatingScore] = useState(5);
  const [categoryRatings, setCategoryRatings] = useState({
    communication: 5,
    reliability: 5,
    quality: 5,
    professionalism: 5,
  });

  useEffect(() => {
    if (currentUser) {
      loadUserStats();
    }
  }, [currentUser]);

  const loadUserStats = async () => {
    if (!currentUser) return;
    setLoading(true);
    try {
      const userStats = await getUserStats(currentUser);
      setStats(userStats);
    } catch (error) {
      console.error('Failed to load stats:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleConnect = async () => {
    try {
      const accounts = await connectWallet();
      if (accounts.length > 0) {
        setCurrentUser(accounts[0].address);
        setConnected(true);
        await createProfile(accounts[0].address);
      }
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const handleSubmitRating = async () => {
    if (!currentUser || !ratingTarget) return;
    try {
      await submitRating(
        currentUser,
        ratingTarget,
        ratingScore,
        categoryRatings.communication,
        categoryRatings.reliability,
        categoryRatings.quality,
        categoryRatings.professionalism
      );
      setRatingTarget('');
      setRatingScore(5);
      setCategoryRatings({
        communication: 5,
        reliability: 5,
        quality: 5,
        professionalism: 5,
      });
      await loadUserStats();
    } catch (error) {
      console.error('Failed to submit rating:', error);
    }
  };

  if (!connected) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-600 flex items-center justify-center p-4">
        <div className="bg-white rounded-2xl shadow-2xl p-8 max-w-md w-full text-center">
          <div className="mb-6">
            <Wallet className="w-16 h-16 mx-auto text-indigo-600 mb-4" />
            <h1 className="text-3xl font-bold text-gray-900 mb-2">ReputeChain</h1>
            <p className="text-gray-600">Your portable reputation across all platforms</p>
          </div>
          <button
            onClick={handleConnect}
            className="w-full bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-bold py-3 px-6 rounded-lg hover:shadow-lg transition-all duration-200"
          >
            Connect Wallet
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-bold text-gray-900">ReputeChain Dashboard</h1>
          <div className="text-sm text-gray-600">
            {currentUser && <span>Connected: {currentUser.slice(0, 10)}...</span>}
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {loading ? (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
          </div>
        ) : stats ? (
          <>
            {/* Stats Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
              {/* Overall Score */}
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-gray-600 text-sm font-medium">Overall Score</p>
                    <p className="text-3xl font-bold text-gray-900 mt-2">
                      {stats.averageScore.toFixed(1)}
                    </p>
                  </div>
                  <Star className="w-12 h-12 text-yellow-400" />
                </div>
              </div>

              {/* Total Reviews */}
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-gray-600 text-sm font-medium">Total Reviews</p>
                    <p className="text-3xl font-bold text-gray-900 mt-2">
                      {stats.totalReviews}
                    </p>
                  </div>
                  <Users className="w-12 h-12 text-blue-400" />
                </div>
              </div>

              {/* Status */}
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-gray-600 text-sm font-medium">Status</p>
                    <p className="text-3xl font-bold text-gray-900 mt-2">
                      {stats.isActive ? 'Active' : 'Inactive'}
                    </p>
                  </div>
                  <Award className="w-12 h-12 text-green-400" />
                </div>
              </div>

              {/* Staked Amount */}
              <div className="bg-white rounded-lg shadow p-6">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-gray-600 text-sm font-medium">Staked</p>
                    <p className="text-3xl font-bold text-gray-900 mt-2">
                      {stats.stakedAmount}
                    </p>
                  </div>
                  <TrendingUp className="w-12 h-12 text-purple-400" />
                </div>
              </div>
            </div>

            {/* Category Breakdown */}
            <div className="bg-white rounded-lg shadow p-6 mb-8">
              <h2 className="text-xl font-bold text-gray-900 mb-6">Category Breakdown</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {Object.entries(stats.categoryAverages).map(([category, score]: [string, any]) => (
                  <div key={category} className="bg-gradient-to-br from-indigo-50 to-purple-50 rounded-lg p-4">
                    <p className="text-gray-700 font-medium capitalize mb-2">{category}</p>
                    <div className="flex items-end gap-2">
                      <p className="text-2xl font-bold text-indigo-600">{score.toFixed(1)}</p>
                      <p className="text-gray-500 text-sm">/5</p>
                    </div>
                    <div className="mt-3 bg-gray-200 rounded-full h-2 overflow-hidden">
                      <div
                        className="bg-gradient-to-r from-indigo-500 to-purple-500 h-full"
                        style={{ width: `${(score / 5) * 100}%` }}
                      ></div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Submit Rating */}
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-xl font-bold text-gray-900 mb-6">Submit a Rating</h2>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Rate User Address
                  </label>
                  <input
                    type="text"
                    value={ratingTarget}
                    onChange={(e) => setRatingTarget(e.target.value)}
                    placeholder="Enter wallet address"
                    className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Overall Score: {ratingScore}
                  </label>
                  <input
                    type="range"
                    min="1"
                    max="5"
                    value={ratingScore}
                    onChange={(e) => setRatingScore(Number(e.target.value))}
                    className="w-full"
                  />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                  {Object.entries(categoryRatings).map(([category, value]: [string, any]) => (
                    <div key={category}>
                      <label className="block text-sm font-medium text-gray-700 mb-2 capitalize">
                        {category}: {value}
                      </label>
                      <input
                        type="range"
                        min="1"
                        max="5"
                        value={value}
                        onChange={(e) =>
                          setCategoryRatings({
                            ...categoryRatings,
                            [category]: Number(e.target.value),
                          })
                        }
                        className="w-full"
                      />
                    </div>
                  ))}
                </div>

                <button
                  onClick={handleSubmitRating}
                  className="w-full bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-bold py-3 px-6 rounded-lg hover:shadow-lg transition-all duration-200"
                >
                  Submit Rating
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="text-center py-12">
            <p className="text-gray-600">No reputation data found. Create a profile first.</p>
          </div>
        )}
      </main>
    </div>
  );
}
