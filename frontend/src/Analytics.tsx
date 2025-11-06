import React, { useState } from 'react';
import { LineChart, Line, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { TrendingUp, BarChart3, PieChart } from 'lucide-react';

export default function Analytics() {
  const [timeRange, setTimeRange] = useState('30d');

  const scoreData = [
    { date: 'Jan 1', score: 3.5 },
    { date: 'Jan 8', score: 3.7 },
    { date: 'Jan 15', score: 3.9 },
    { date: 'Jan 22', score: 4.1 },
    { date: 'Jan 29', score: 4.3 },
    { date: 'Feb 5', score: 4.5 },
    { date: 'Feb 12', score: 4.7 },
  ];

  const categoryData = [
    { category: 'Communication', score: 4.8 },
    { category: 'Reliability', score: 4.6 },
    { category: 'Quality', score: 4.7 },
    { category: 'Professionalism', score: 4.9 },
  ];

  const platformData = [
    { name: 'Uber', value: 45, color: '#000' },
    { name: 'Airbnb', value: 30, color: '#FF5A5F' },
    { name: 'Upwork', value: 25, color: '#14A800' },
  ];

  const recentReviews = [
    {
      reviewer: '5A6B7C8D9E0F1G2H',
      score: 5,
      comment: 'Excellent service, very professional',
      date: '2 hours ago',
    },
    {
      reviewer: '6B7C8D9E0F1G2H3I',
      score: 4,
      comment: 'Good experience, minor delays',
      date: '1 day ago',
    },
    {
      reviewer: '7C8D9E0F1G2H3I4J',
      score: 5,
      comment: 'Outstanding work quality',
      date: '3 days ago',
    },
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <h1 className="text-2xl font-bold text-gray-900">Analytics & Insights</h1>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Time Range Selector */}
        <div className="mb-6 flex gap-2">
          {['7d', '30d', '90d', '1y'].map((range) => (
            <button
              key={range}
              onClick={() => setTimeRange(range)}
              className={`px-4 py-2 rounded-lg font-medium transition-all ${
                timeRange === range
                  ? 'bg-indigo-600 text-white'
                  : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
              }`}
            >
              {range === '7d' ? '7 Days' : range === '30d' ? '30 Days' : range === '90d' ? '90 Days' : '1 Year'}
            </button>
          ))}
        </div>

        {/* Score Trend Chart */}
        <div className="bg-white rounded-lg shadow p-6 mb-8">
          <div className="flex items-center gap-2 mb-6">
            <TrendingUp className="w-5 h-5 text-indigo-600" />
            <h2 className="text-xl font-bold text-gray-900">Score Trend</h2>
          </div>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={scoreData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="date" />
              <YAxis domain={[0, 5]} />
              <Tooltip />
              <Legend />
              <Line
                type="monotone"
                dataKey="score"
                stroke="#6366f1"
                strokeWidth={2}
                dot={{ fill: '#6366f1', r: 5 }}
                activeDot={{ r: 7 }}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Category Performance */}
        <div className="bg-white rounded-lg shadow p-6 mb-8">
          <div className="flex items-center gap-2 mb-6">
            <BarChart3 className="w-5 h-5 text-indigo-600" />
            <h2 className="text-xl font-bold text-gray-900">Category Performance</h2>
          </div>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={categoryData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="category" />
              <YAxis domain={[0, 5]} />
              <Tooltip />
              <Bar dataKey="score" fill="#6366f1" radius={[8, 8, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Platform Breakdown & Recent Reviews */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Platform Distribution */}
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center gap-2 mb-6">
              <PieChart className="w-5 h-5 text-indigo-600" />
              <h2 className="text-xl font-bold text-gray-900">Platform Distribution</h2>
            </div>
            <div className="space-y-4">
              {platformData.map((platform, idx) => (
                <div key={idx}>
                  <div className="flex justify-between mb-2">
                    <span className="font-medium text-gray-700">{platform.name}</span>
                    <span className="text-gray-600">{platform.value}%</span>
                  </div>
                  <div className="bg-gray-200 rounded-full h-2 overflow-hidden">
                    <div
                      className="bg-indigo-600 h-full"
                      style={{ width: `${platform.value}%` }}
                    ></div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Recent Reviews */}
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-xl font-bold text-gray-900 mb-6">Recent Reviews</h2>
            <div className="space-y-4">
              {recentReviews.map((review, idx) => (
                <div key={idx} className="border-b border-gray-200 pb-4 last:border-0">
                  <div className="flex justify-between items-start mb-2">
                    <span className="text-sm font-mono text-gray-600">
                      {review.reviewer.slice(0, 10)}...
                    </span>
                    <span className="text-xs text-gray-500">{review.date}</span>
                  </div>
                  <div className="flex gap-1 mb-2">
                    {Array(review.score)
                      .fill(0)
                      .map((_, i) => (
                        <span key={i} className="text-yellow-400">⭐</span>
                      ))}
                  </div>
                  <p className="text-sm text-gray-700">{review.comment}</p>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Key Metrics */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
          <div className="bg-gradient-to-br from-indigo-50 to-purple-50 rounded-lg p-6">
            <p className="text-gray-600 text-sm font-medium mb-2">Avg Response Time</p>
            <p className="text-3xl font-bold text-indigo-600">2.3h</p>
            <p className="text-xs text-gray-500 mt-2">↓ 15% from last month</p>
          </div>
          <div className="bg-gradient-to-br from-green-50 to-emerald-50 rounded-lg p-6">
            <p className="text-gray-600 text-sm font-medium mb-2">Completion Rate</p>
            <p className="text-3xl font-bold text-green-600">98.5%</p>
            <p className="text-xs text-gray-500 mt-2">↑ 2% from last month</p>
          </div>
          <div className="bg-gradient-to-br from-orange-50 to-yellow-50 rounded-lg p-6">
            <p className="text-gray-600 text-sm font-medium mb-2">Repeat Customers</p>
            <p className="text-3xl font-bold text-orange-600">67%</p>
            <p className="text-xs text-gray-500 mt-2">↑ 8% from last month</p>
          </div>
        </div>
      </main>
    </div>
  );
}
