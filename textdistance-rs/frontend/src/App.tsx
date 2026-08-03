import { useState, useEffect } from 'react';

interface CompareResult {
  category: string;
  algorithm: string;
  distance: number;
  similarity: number;
  normalized_distance: number;
  normalized_similarity: number;
}

interface CompareAllResponse {
  results: CompareResult[];
}

function App() {
  const [s1, setS1] = useState('kitten');
  const [s2, setS2] = useState('sitting');
  const [results, setResults] = useState<CompareResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchResults = async () => {
      if (!s1 || !s2) {
        setResults([]);
        return;
      }
      
      setLoading(true);
      setError('');
      
      try {
        const apiBase = import.meta.env.VITE_API_URL ?? '';
        const response = await fetch(`${apiBase}/api/compare_all`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ s1, s2 }),
        });
        
        if (!response.ok) {
          throw new Error('Failed to fetch results');
        }
        
        const data: CompareAllResponse = await response.json();
        setResults(data.results);
      } catch (err) {
        console.error(err);
        setError('Failed to connect to API. Is the server running?');
      } finally {
        setLoading(false);
      }
    };

    const timer = setTimeout(() => {
      fetchResults();
    }, 300);

    return () => clearTimeout(timer);
  }, [s1, s2]);

  // Group results by category
  const groupedResults = results.reduce((acc, result) => {
    if (!acc[result.category]) {
      acc[result.category] = [];
    }
    acc[result.category].push(result);
    return acc;
  }, {} as Record<string, CompareResult[]>);

  const getScoreColorClass = (score: number) => {
    if (score >= 0.8) return 'score-high';
    if (score >= 0.5) return 'score-med';
    return 'score-low';
  };

  const getScoreColorHex = (score: number) => {
    if (score >= 0.8) return '#10b981';
    if (score >= 0.5) return '#f59e0b';
    return '#ef4444';
  };

  return (
    <div className="app-container">
      <header className="header">
        <h1>TextDistance</h1>
        <p>36 Algorithms. 0 Dependencies. 100% Rust.</p>
      </header>

      <section className="input-section">
        <div className="input-group">
          <label htmlFor="s1">String A</label>
          <input
            id="s1"
            type="text"
            value={s1}
            onChange={(e) => setS1(e.target.value)}
            placeholder="Enter first string..."
            autoComplete="off"
          />
        </div>
        <div className="input-group">
          <label htmlFor="s2">String B</label>
          <input
            id="s2"
            type="text"
            value={s2}
            onChange={(e) => setS2(e.target.value)}
            placeholder="Enter second string..."
            autoComplete="off"
          />
        </div>
      </section>

      {error && (
        <div className="glass-panel" style={{ borderLeft: '4px solid var(--danger)' }}>
          <p style={{ color: 'var(--danger)', margin: 0 }}>{error}</p>
        </div>
      )}

      {!s1 || !s2 ? (
        <div className="glass-panel empty-state">
          <svg fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
          <p>Enter two strings to compare them across all algorithms instantly.</p>
        </div>
      ) : (
        <div className="results-container">
          {loading && <div className="glass-panel" style={{textAlign:'center',color:'var(--text-muted)'}}>Computing...</div>}
          {Object.entries(groupedResults).map(([category, items]) => (
            <div key={category} className="category-section">
              <h2 className="category-title">{category} Algorithms</h2>
              <div className="results-grid">
                {items.map((item) => {
                  const displayScore = Number.isFinite(item.normalized_similarity) ? item.normalized_similarity : 0;
                  const percent = Math.max(0, Math.min(100, displayScore * 100));
                  
                  return (
                    <div key={item.algorithm} className="glass-panel metric-card">
                      <div className="metric-header">
                        <h3 className="metric-title">{item.algorithm}</h3>
                      </div>
                      
                      <div className={`metric-score ${getScoreColorClass(displayScore)}`}>
                        {percent.toFixed(1)}%
                      </div>
                      
                      <div className="progress-bg">
                        <div 
                          className="progress-fill" 
                          style={{ 
                            width: `${percent}%`,
                            backgroundColor: getScoreColorHex(displayScore)
                          }}
                        ></div>
                      </div>
                      
                      <div className="metric-details">
                        <span>Dist: {Number.isFinite(item.distance) ? item.distance.toFixed(2) : 'N/A'}</span>
                        <span>Sim: {Number.isFinite(item.similarity) ? item.similarity.toFixed(2) : 'N/A'}</span>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
