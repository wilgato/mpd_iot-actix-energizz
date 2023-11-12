// static/js/script.js

// Função para buscar dados da API
async function fetchData() {
    const response = await fetch('http://127.0.0.1:8000/energizz');
    const data = await response.json();
    return data;
}

// Função para formatar os dados para o Chart.js
function formatData(data) {
    return {
        labels: data.map(entry => entry.posting_time),
        datasets: [
            {
                label: 'Temperatura',
                data: data.map(entry => entry.temperatura),
                borderColor: 'rgba(255, 99, 132, 1)',
                borderWidth: 1,
                fill: false,
            },
            // Adicione mais datasets conforme necessário para outras métricas
        ],
    };
}

// Função para inicializar o gráfico
async function initChart() {
    const data = await fetchData();
    const formattedData = formatData(data);

    const ctx = document.getElementById('myChart').getContext('2d');
    new Chart(ctx, {
        type: 'line',
        data: formattedData,
    });
}

// Chame a função de inicialização quando a página estiver pronta
document.addEventListener('DOMContentLoaded', initChart);
