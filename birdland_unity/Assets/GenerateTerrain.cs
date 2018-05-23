using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;

[System.Serializable]
class Tile
{
    public float alt;
    public string obj;

}

[System.Serializable]
class Chunk
{
    public static int SIZE = 64;
    public Tile[] cells = new Tile[64 * 64];

    public static Chunk FromJSON(string json)
    {
        return JsonUtility.FromJson<Chunk>(json);
    }
}

[RequireComponent(typeof(MeshFilter))]
public class GenerateTerrain : MonoBehaviour
{
    Mesh mesh;

    public GameObject tree;

    public int coordX = 1;
    public int coordY = 1;

    List<Vector3> vertices = new List<Vector3>();
    List<int> triangles = new List<int>();
    List<Color32> colors = new List<Color32>();

    void Start()
    {
        mesh = GetComponent<MeshFilter>().mesh;
        StartCoroutine(GetChunk());
    }

    void OnValidate()
    {
        if (Application.isPlaying)
        {
            StartCoroutine(GetChunk());
        }
    }

    void UpdateMesh()
    {
        mesh.Clear();
        mesh.vertices = vertices.ToArray();
        mesh.triangles = triangles.ToArray();
        mesh.colors32 = colors.ToArray();
        mesh.RecalculateNormals();
    }

    void GenerateMesh(Chunk chunk)
    {
        vertices.Clear();
        triangles.Clear();
        colors.Clear();

        for (int i = 0; i < chunk.cells.Length; i++)
        {
            int z = i % Chunk.SIZE;
            int x = i / Chunk.SIZE;
            float y = chunk.cells[i].alt;
            Vector3 position = new Vector3(x, y, z);
            vertices.Add(position);

            if (y < 2)
            {
                colors.Add(new Color32(99, 189, 250, 255));
            }
            else if (y < 3)
            {
                colors.Add(new Color32(190, 180, 157, 255));
            }
            else if (y < 8)
            {
                colors.Add(new Color32(104, 153, 102, 255));
            }
            else
            {
                colors.Add(new Color32(255, 255, 255, 255));
            }

            // Place a tree
            if (chunk.cells[i].obj == "Tree")
            {
                Instantiate(tree, position, Quaternion.identity);
            }
        }

        // Fill the triangles
        for (int i = 0; i < Chunk.SIZE - 1; i++)
        {
            for (int j = 0; j < Chunk.SIZE - 1; j++)
            {
                int topLeft = i * Chunk.SIZE + j;
                int topRight = i * Chunk.SIZE + j + 1;
                int bottomLeft = (i + 1) * Chunk.SIZE + j;
                int bottomRight = (i + 1) * Chunk.SIZE + j + 1;
                triangles.Add(topLeft);
                triangles.Add(topRight);
                triangles.Add(bottomLeft);
                triangles.Add(topRight);
                triangles.Add(bottomRight);
                triangles.Add(bottomLeft);
            }
        }
    }

    IEnumerator GetChunk()
    {
        string url = string.Format("http://localhost:6767/chunk/{0}/{1}", coordX, coordY);
        UnityWebRequest www = UnityWebRequest.Get(url);
        yield return www.SendWebRequest();
        Chunk chunk = Chunk.FromJSON(www.downloadHandler.text);

        GenerateMesh(chunk);
        UpdateMesh();
    }
}
