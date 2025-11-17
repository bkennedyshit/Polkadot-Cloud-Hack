import { useState, useEffect } from 'react';
import { Trophy, Medal, Award } from 'lucide-react';

export default function Leaderboard() {
  const [users, setUsers] = useState<any[]>([]);
  const [filter, setFilter] = useState('all');

  useEffect(() => {
    // Mock data - in production would fetch from API
    const mockUsers = [
      {
        rank: 1,
        address: '1A2B3C4D5E6F7G8H9I0J',
        score: 4.9,
        reviews: 156,
        category: 'driver',
      },
      {
        rank: 2,
        address: '2B3C4D5E6F7G8H9I0J1K',
        score: 4.8,
        reviews: 142,
        category: 'tutor',
      },
      {
        rank: 3,
        address: '3C4D5E6F7G8H9I0J1K2L',
        score: 4.7,
        reviews: 128,
        category: 'freelancer',
      },
    ];
    setUsers(mockUsers);
  }, []);

  const getMedalIcon = (rank: number) => {
    if (rank === 1) return <Trophy className="w-6 h-6 text-yellow-500" />;
    if (rank === 2) return <Medal className="w-6 h-6 text-gray-400" />;
    if (rank === 3) return <Medal className="w-6 h-6 text-orange-600" />;
    return <Award className="w-6 h-6 text-gray-300" />;
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <h1 className="text-2xl font-bold text-gray-900">Global Leaderboard</h1>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Top 3 Podium */}
        <div className="mb-12">
          <h2 className="text-xl font-bold text-gray-900 mb-6">Top Performers</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {users.slice(0, 3).map((user, idx) => (
              <div
                key={idx}
                className={`rounded-lg shadow-lg p-6 text-center ${
                  idx === 0
                    ? 'bg-gradient-to-br from-yellow-400 to-yellow-600 text-white'
                    : idx === 1
                    ? 'bg-gradient-to-br from-gray-300 to-gray-500 text-white'
                    : 'bg-gradient-to-br from-orange-400 to-orange-600 text-white'
                }`}
              >
                <div className="flex justify-center mb-4">
                  {getMedalIcon(user.rank)}
                </div>
                <p className="text-sm font-medium opacity-90">#{user.rank}</p>
                <p className="text-2xl font-bold mt-2">{user.score}</p>
                <p className="text-sm opacity-90 mt-1">{user.reviews} reviews</p>
              </div>
            ))}
          </div>
        </div>

        {/* Filters */}
        <div className="mb-6 flex gap-2">
          {['all', 'driver', 'tutor', 'freelancer'].map((cat) => (
            <button
              key={cat}
              onClick={() => setFilter(cat)}
              className={`px-4 py-2 rounded-lg font-medium transition-all ${
                filter === cat
                  ? 'bg-indigo-600 text-white'
                  : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
              }`}
            >
              {cat.charAt(0).toUpperCase() + cat.slice(1)}
            </button>
          ))}
        </div>

        {/* Full Leaderboard Table */}
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="w-full">
            <thead className="bg-gray-50 border-b border-gray-200">
              <tr>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-900">Rank</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-900">Address</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-900">Score</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-900">Reviews</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-900">Category</th>
              </tr>
            </thead>
            <tbody>
              {users.map((user, idx) => (
                <tr key={idx} className="border-b border-gray-200 hover:bg-gray-50">
                  <td className="px-6 py-4 text-sm font-semibold text-gray-900">
                    <div className="flex items-center gap-2">
                      {getMedalIcon(user.rank)}
                      #{user.rank}
                    </div>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-600 font-mono">
                    {user.address.slice(0, 10)}...
                  </td>
                  <td className="px-6 py-4 text-sm font-semibold text-gray-900">
                    {user.score} ⭐
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-600">{user.reviews}</td>
                  <td className="px-6 py-4 text-sm">
                    <span className="px-3 py-1 bg-indigo-100 text-indigo-700 rounded-full text-xs font-medium">
                      {user.category}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
}
