import { useState } from 'react';
import Dashboard from './Dashboard';
import Leaderboard from './Leaderboard';
import Analytics from './Analytics';
import { BarChart3, Trophy, TrendingUp } from 'lucide-react';

export default function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Navigation */}
      <nav className="bg-white shadow-sm border-b border-gray-200 sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-gradient-to-br from-indigo-600 to-purple-600 rounded-lg flex items-center justify-center">
                <span className="text-white font-bold text-lg">⭐</span>
              </div>
              <span className="text-xl font-bold text-gray-900">ReputeChain</span>
            </div>

            <div className="flex gap-1">
              <button
                onClick={() => setCurrentPage('dashboard')}
                className={`px-4 py-2 rounded-lg font-medium transition-all flex items-center gap-2 ${
                  currentPage === 'dashboard'
                    ? 'bg-indigo-100 text-indigo-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                <BarChart3 className="w-4 h-4" />
                Dashboard
              </button>
              <button
                onClick={() => setCurrentPage('leaderboard')}
                className={`px-4 py-2 rounded-lg font-medium transition-all flex items-center gap-2 ${
                  currentPage === 'leaderboard'
                    ? 'bg-indigo-100 text-indigo-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                <Trophy className="w-4 h-4" />
                Leaderboard
              </button>
              <button
                onClick={() => setCurrentPage('analytics')}
                className={`px-4 py-2 rounded-lg font-medium transition-all flex items-center gap-2 ${
                  currentPage === 'analytics'
                    ? 'bg-indigo-100 text-indigo-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                <TrendingUp className="w-4 h-4" />
                Analytics
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* Page Content */}
      {currentPage === 'dashboard' && <Dashboard />}
      {currentPage === 'leaderboard' && <Leaderboard />}
      {currentPage === 'analytics' && <Analytics />}
    </div>
  );
}
