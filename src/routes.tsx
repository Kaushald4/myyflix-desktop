import { Route, Routes } from "react-router";
import Home from "./pages/Home";
import Layout from "./components/global/Layout";
import MoviesPage from "./pages/Movies";
import SeriesPage from "./pages/Series";
import HistoryPage from "./pages/History";
import WatchlistPage from "./pages/WatchList";
import Downloads from "./pages/Downloads";
import WatchPage from "./pages/WatchPage";
import DetailsPage from "./pages/DetailsPage";

const AppRoutes = () => {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/movies" element={<MoviesPage />} />
        <Route path="/series" element={<SeriesPage />} />
        <Route path="/history" element={<HistoryPage />} />
        <Route path="/watchlist" element={<WatchlistPage />} />
        <Route path="/downloads" element={<Downloads />} />
        <Route path="/:type/:id" element={<DetailsPage />} />

        <Route path="/watch/:type/:id" element={<WatchPage />} />
      </Routes>
    </Layout>
  );
};

export default AppRoutes;
