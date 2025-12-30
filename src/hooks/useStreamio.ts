import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import {
  fetchContent,
  fetchMetaDetails,
  searchContent,
  ContentType,
} from "@/lib/api";

export const useInfiniteContent = (type: ContentType, genre?: string) => {
  return useInfiniteQuery({
    queryKey: ["content", type, genre],
    queryFn: async ({ pageParam = 0 }) => {
      const skip = typeof pageParam === "number" ? pageParam : 0;
      const data = await fetchContent({ type, skip, genre });
      return Array.isArray(data) ? data : [];
    },
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      const last = Array.isArray(lastPage) ? lastPage : [];
      if (last.length === 0) return undefined;

      const totalLoaded = allPages.reduce((acc, page) => {
        return acc + (Array.isArray(page) ? page.length : 0);
      }, 0);

      return totalLoaded;
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
};

export const useContentDetails = (type: string, id: string) => {
  return useQuery({
    queryKey: ["details", type, id],
    queryFn: () => fetchMetaDetails(type, id),
    enabled: !!type && !!id,
    staleTime: 1000 * 60 * 30, // 30 minutes
  });
};

export const useSearchContent = (query: string, type: ContentType) => {
  return useQuery({
    queryKey: ["search", type, query],
    queryFn: () => searchContent(query, type),
    enabled: !!query,
    staleTime: 1000 * 60 * 5, // 5 minutes
  });
};
