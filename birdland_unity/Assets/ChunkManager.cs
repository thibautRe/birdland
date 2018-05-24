using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ChunkManager : MonoBehaviour
{
    public ChunkGenerator chunkObject;

    // Use this for initialization
    void Start()
    {
        InstantiateChunksAround(0, 0, 1);
    }


    void InstantiateChunk(int x, int y)
    {
        ChunkGenerator chunk = Instantiate(chunkObject, new Vector3(64 * x, 0, -64 * y), Quaternion.identity);
        chunk.coordX = x;
        chunk.coordY = y;
    }

    // Size represents the number of chunks to be loaded around a certain
    // position. Ex: size = 1 => 9 chunks. size = 2 => 25 chunks
    void InstantiateChunksAround(int x, int y, int size)
    {
        for (int chunkx = x - size; chunkx <= x + size; chunkx++)
        {
            for (int chunky = y - size; chunky <= y + size; chunky++)
            {
                InstantiateChunk(chunkx, chunky);
            }
        }
    }
}
