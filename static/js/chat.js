// Importe o Chart.js no topo do seu chart.js
import Chart from 'chart.js/auto';

// Crie o gráfico no início do seu chart.js
const ctx = document.getElementById('chart').getContext('2d');
const chart = new Chart(ctx, {
    type: 'line',
    data: {
        labels: [], // Use isso para as etiquetas do eixo X (por exemplo, timestamps)
        datasets: [{
            label: 'Batimentos Cardíacos',
            data: [], // Use isso para os dados do eixo Y (por exemplo, frequência cardíaca)
            borderColor: 'rgb(75, 192, 192)',
            fill: false,
        }]
    },
    options: {
        responsive: true,
        scales: {
            x: {
                type: 'linear',
                position: 'bottom',
                title: {
                    display: true,
                    text: 'Tempo'
                }
            },
            y: {
                beginAtZero: true,
                title: {
                    display: true,
                    text: 'Batimentos Cardíacos (BPM)'
                }
            }
        }
    }
});
